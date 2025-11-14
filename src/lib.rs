#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::ptr;

mod bitwise;
mod hash;
mod eq;
mod index;

pub type Bit = bool;

type Word = usize;

const BITS_PER_WORD: usize = Word::BITS as usize;

#[derive(Debug, Default)]
pub struct BitVec {
    buf: Box<[Word]>,
    len: usize,
}

impl BitVec {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len().saturating_mul(BITS_PER_WORD)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl BitVec {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let words = Self::words_needed(capacity);
        let buf = Self::allocate(words);
        let len = 0;
        Self { buf, len }
    }
}

impl BitVec {
    pub fn push(&mut self, value: Bit) -> &mut Self {
        self.reserve(1);
        unsafe {
            self.set_unchecked(self.len, value);
        }
        self.len += 1;
        self
    }

    pub fn pop(&mut self) -> Option<Bit> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(unsafe { self.get_unchecked(self.len) })
        }
    }
}

impl BitVec {
    pub fn reserve(&mut self, additional: usize) -> &mut Self {
        let Some(capacity) = self.len.checked_add(additional) else {
            panic!("capacity overflow");
        };
        let words = Self::words_needed(capacity);
        if words <= self.buf.len() {
            return self;
        }
        let words = self.buf.len().saturating_mul(2).max(words).max(4);
        unsafe { self.reallocate(words) }
    }

    pub fn reserve_exact(&mut self, additional: usize) -> &mut Self {
        let Some(capacity) = self.len.checked_add(additional) else {
            panic!("capacity overflow");
        };
        let words = Self::words_needed(capacity);
        if words <= self.buf.len() {
            return self;
        }
        unsafe { self.reallocate(words) }
    }

    pub fn shrink_to_fit(&mut self) -> &mut Self {
        let words = Self::words_needed(self.len);
        if words == self.buf.len() {
            return self;
        }
        unsafe { self.reallocate(words) }
    }

    pub fn shrink_to(&mut self, capacity: usize) -> &mut Self {
        let capacity = self.len.max(capacity);
        let words = Self::words_needed(capacity);
        if words >= self.buf.len() {
            return self;
        }
        unsafe { self.reallocate(words) }
    }

    fn allocate(words: usize) -> Box<[Word]> {
        // SAFETY: `Word` has no invalid bit patterns and does not need to drop,
        // so it is safe to assume uninitialized memory as initialized.
        unsafe { Box::new_uninit_slice(words).assume_init() }
    }

    /// # Safety
    ///
    /// Caller must ensure that `Self::words_needed(self.len) <= words`.
    unsafe fn reallocate(&mut self, words: usize) -> &mut Self {
        let mut buf = Self::allocate(words);
        let src = self.buf.as_ptr();
        let dst = buf.as_mut_ptr();
        let count = Self::words_needed(self.len);
        unsafe {
            ptr::copy_nonoverlapping(src, dst, count);
        }
        self.buf = buf;
        self
    }
}

impl BitVec {
    fn words_needed(bits: usize) -> usize {
        if bits == 0 {
            0
        } else {
            (bits - 1) / BITS_PER_WORD + 1
        }
    }
}
