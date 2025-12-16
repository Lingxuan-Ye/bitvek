#![no_std]

extern crate alloc;

pub use self::primitive::{Bit, Byte};

use self::primitive::Word;
use alloc::vec::Vec;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Index;

mod bitwise;
mod convert;
mod iter;
mod macros;
mod primitive;

#[cfg(feature = "serde")]
mod serde;

#[derive(Default)]
pub struct BitVec {
    // Invariant: `self.buf_used() <= self.buf.len()`
    len: usize,
    buf: Vec<Word>,
}

impl BitVec {
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.buf.capacity().saturating_mul(Word::BITS)
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    const fn buf_used(&self) -> usize {
        self.len.div_ceil(Word::BITS)
    }
}

impl BitVec {
    #[inline]
    pub const fn new() -> Self {
        let len = 0;
        let buf = Vec::new();
        Self { len, buf }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let len = 0;
        let buf_capacity = capacity.div_ceil(Word::BITS);
        let buf = Vec::with_capacity(buf_capacity);
        Self { len, buf }
    }
}

impl BitVec {
    pub fn reserve(&mut self, additional: usize) -> &mut Self {
        let capacity = self.len.checked_add(additional).expect("capacity overflow");
        let buf_capacity = capacity.div_ceil(Word::BITS);
        if let Some(buf_additional) = buf_capacity.checked_sub(self.buf.len()) {
            self.buf.reserve(buf_additional);
        };
        self
    }

    pub fn reserve_exact(&mut self, additional: usize) -> &mut Self {
        let capacity = self.len.checked_add(additional).expect("capacity overflow");
        let buf_capacity = capacity.div_ceil(Word::BITS);
        if let Some(buf_additional) = buf_capacity.checked_sub(self.buf.len()) {
            self.buf.reserve_exact(buf_additional);
        };
        self
    }

    pub fn shrink_to_fit(&mut self) -> &mut Self {
        let buf_new_len = self.buf_used();
        unsafe {
            self.buf.set_len(buf_new_len);
        }
        self.buf.shrink_to_fit();
        self
    }

    pub fn shrink_to(&mut self, min_capacity: usize) -> &mut Self {
        let buf_min_capacity = min_capacity.div_ceil(Word::BITS);
        if buf_min_capacity < self.buf.len() {
            let buf_new_len = self.buf_used().max(buf_min_capacity);
            unsafe {
                self.buf.set_len(buf_new_len);
            }
            self.buf.shrink_to_fit();
        } else if buf_min_capacity < self.buf.capacity() {
            self.buf.shrink_to(buf_min_capacity);
        }
        self
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<Bit> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> Bit {
        let loc = Loc::new(index);
        let word = unsafe { self.buf.get_unchecked(loc.period) };
        word.get(loc.offset)
    }

    #[inline]
    #[must_use]
    pub fn set(&mut self, index: usize, value: Bit) -> Option<&mut Self> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.set_unchecked(index, value) })
        }
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: Bit) -> &mut Self {
        let loc = Loc::new(index);
        let word = unsafe { self.buf.get_unchecked_mut(loc.period) };
        word.set(loc.offset, value);
        self
    }

    pub fn push(&mut self, value: Bit) -> &mut Self {
        if self.len == usize::MAX {
            panic!("capacity overflow")
        }
        let loc = Loc::new(self.len);
        if loc.period < self.buf.len() {
            let word = unsafe { self.buf.get_unchecked_mut(loc.period) };
            word.set(loc.offset, value);
        } else if value {
            self.buf.push(Word::MSB_SET);
        } else {
            self.buf.push(Word::MSB_CLEAR);
        }
        self.len += 1;
        self
    }

    pub fn pop(&mut self) -> Option<Bit> {
        if self.is_empty() {
            return None;
        }
        self.len -= 1;
        let loc = Loc::new(self.len);
        let word = unsafe { self.buf.get_unchecked(loc.period) };
        let value = word.get(loc.offset);
        Some(value)
    }
}

impl Index<usize> for BitVec {
    type Output = Bit;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            None => panic!("index out of bounds"),
            Some(false) => &false,
            Some(true) => &true,
        }
    }
}

impl Clone for BitVec {
    fn clone(&self) -> Self {
        let len = self.len;
        let buf_len = self.buf_used();
        let buf = unsafe { self.buf.get_unchecked(0..buf_len).to_vec() };
        Self { len, buf }
    }
}

impl fmt::Debug for BitVec {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Extend<Bit> for BitVec {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Bit>,
    {
        let iter = iter.into_iter();
        let additional = iter.size_hint().0;
        self.reserve(additional);
        for value in iter {
            self.push(value);
        }
    }
}

impl Hash for BitVec {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        if self.is_empty() {
            return;
        }

        self.len.hash(state);

        let last = self.len - 1;
        let loc = Loc::new(last);

        let head = unsafe { self.buf.get_unchecked(..loc.period) };
        head.hash(state);

        let tail = unsafe { self.buf.get_unchecked(loc.period) };
        tail.align_last_to_lsb(loc.offset).hash(state);
    }
}

impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        if self.is_empty() {
            return true;
        }

        let last = self.len - 1;
        let loc = Loc::new(last);

        let lhs_head = unsafe { self.buf.get_unchecked(..loc.period) };
        let rhs_head = unsafe { other.buf.get_unchecked(..loc.period) };
        if lhs_head != rhs_head {
            return false;
        }

        let lhs_tail = unsafe { self.buf.get_unchecked(loc.period) };
        let rhs_tail = unsafe { other.buf.get_unchecked(loc.period) };
        (*lhs_tail ^ *rhs_tail).align_last_to_lsb(loc.offset) == Word::CLEAR
    }
}

impl Eq for BitVec {}

#[derive(Debug)]
struct Loc {
    period: usize,
    offset: usize,
}

impl Loc {
    const fn new(index: usize) -> Self {
        let period = index / Word::BITS;
        let offset = index % Word::BITS;
        Self { period, offset }
    }
}
