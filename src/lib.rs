//! Say, we have a bit vector —
//!
//! it's nothing better than a `Vec<bool>`, but …
//!
//! what if we implement it,
//!
//! and save some poor bits of memory?
//!
//! # Quick Start
//!
//! ```
//! use bitvek::bitvec;
//!
//! let vec = bitvec![
//!     true, true, true, true, false, false, false, false,
//!     false, false, false, false, true, true, true, true,
//! ];
//! ```
//!
//! Find it cumbersome? Try this:
//!
//! ```
//! # use bitvek::bitvec;
//! #
//! // The total number of bits must be a multiple of 8.
//! let vec = bitvec![0b11110000, 0b00001111];
//! ```

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

/// A bit vector.
#[derive(Default)]
pub struct BitVec {
    // Invariant: `self.buf_used() <= self.buf.len()`
    len: usize,
    buf: Vec<Word>,
}

impl BitVec {
    /// Returns the total number of bits the vector can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let vec = BitVec::with_capacity(10);
    /// assert!(vec.capacity() >= 10);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.buf.capacity().saturating_mul(Word::BITS)
    }

    /// Returns the number of bits in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let vec = bitvec![true, true, false, false];
    /// assert_eq!(vec.len(), 4);
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector contains no bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let vec = bitvec![];
    /// assert!(vec.is_empty());
    ///
    /// let vec = bitvec![true, true, false, false];
    /// assert!(!vec.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of words used to store the bits in the vector.
    ///
    /// Note that it is always less than or equal to the buffer length, which
    /// represents the number of words initialized.
    const fn buf_used(&self) -> usize {
        self.len.div_ceil(Word::BITS)
    }
}

impl BitVec {
    /// Creates a new, empty [`BitVec`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let vec = BitVec::new();
    /// assert_eq!(vec.len(), 0);
    /// assert_eq!(vec.capacity(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        let len = 0;
        let buf = Vec::new();
        Self { len, buf }
    }

    /// Creates a new, empty [`BitVec`] with the specified capacity.
    ///
    /// The vector will be able to hold at least `capacity` bits without
    /// reallocating. This method is allowed to allocate for more bits than
    /// `capacity`. If `capacity` is zero, the vector will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let vec = BitVec::with_capacity(10);
    /// assert_eq!(vec.len(), 0);
    /// assert!(vec.capacity() >= 10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let len = 0;
        let buf_capacity = capacity.div_ceil(Word::BITS);
        let buf = Vec::with_capacity(buf_capacity);
        Self { len, buf }
    }
}

impl BitVec {
    /// Reserves capacity for at least `additional` more bits to be inserted in the
    /// given [`BitVec`]. The collection may reserve more space to speculatively
    /// avoid frequent reallocations. After calling `reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if capacity
    /// is already sufficient.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// vec.reserve(6);
    /// assert!(vec.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) -> &mut Self {
        let capacity = self.len.checked_add(additional).expect("capacity overflow");
        let buf_capacity = capacity.div_ceil(Word::BITS);
        if let Some(buf_additional) = buf_capacity.checked_sub(self.buf.len()) {
            self.buf.reserve(buf_additional);
        };
        self
    }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// let unchanged = vec.clone();
    ///
    /// vec.reserve(6);
    /// assert!(vec.capacity() >= 10);
    ///
    /// vec.shrink_to_fit();
    /// assert!(vec.capacity() >= 4);
    /// assert_eq!(vec, unchanged);
    /// ```
    pub fn shrink_to_fit(&mut self) -> &mut Self {
        let buf_new_len = self.buf_used();
        unsafe {
            self.buf.set_len(buf_new_len);
        }
        self.buf.shrink_to_fit();
        self
    }

    /// Shrinks the capacity of the vector with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the
    /// supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// let unchanged = vec.clone();
    ///
    /// vec.reserve(6);
    /// assert!(vec.capacity() >= 10);
    ///
    /// vec.shrink_to(8);
    /// assert!(vec.capacity() >= 8);
    /// assert_eq!(vec, unchanged);
    ///
    /// vec.shrink_to(0);
    /// assert!(vec.capacity() >= 4);
    /// assert_eq!(vec, unchanged);
    /// ```
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

    /// Returns the bit at the specified index, if in bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let vec = bitvec![true, true, false, false];
    /// assert_eq!(vec.get(3), Some(false));
    /// assert_eq!(vec.get(4), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<Bit> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    /// Returns the bit at the specified index, without performing any bounds
    /// checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let vec = bitvec![true, true, false, false];
    /// assert_eq!(unsafe { vec.get_unchecked(3) }, false);
    /// ```
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> Bit {
        let loc = Loc::new(index);
        let word = unsafe { self.buf.get_unchecked(loc.period) };
        word.get(loc.offset)
    }

    /// Sets the bit at the specified index to the specified value, if in bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// assert!(vec.set(2, true).is_some());
    /// assert!(vec.set(3, true).is_some());
    /// assert!(vec.set(4, true).is_none());
    /// assert_eq!(vec, bitvec![true; 4]);
    /// ```
    #[inline]
    #[must_use]
    pub fn set(&mut self, index: usize, value: Bit) -> Option<&mut Self> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.set_unchecked(index, value) })
        }
    }

    /// Sets the bit at the specified index to the specified value, without
    /// performing any bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// unsafe {
    ///     vec.set_unchecked(2, true);
    ///     vec.set_unchecked(3, true);
    /// }
    /// assert_eq!(vec, bitvec![true; 4]);
    /// ```
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: Bit) -> &mut Self {
        let loc = Loc::new(index);
        let word = unsafe { self.buf.get_unchecked_mut(loc.period) };
        word.set(loc.offset, value);
        self
    }

    /// Appends a bit to the back of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the required capacity exceeds `usize::MAX` bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// vec.push(true);
    /// assert_eq!(vec, bitvec![true, true, false, false, true]);
    /// ```
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

    /// Removes the last bit from the vector and returns it, or `None` if the vector
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// assert_eq!(vec.pop(), Some(false));
    /// assert_eq!(vec.pop(), Some(false));
    /// assert_eq!(vec.pop(), Some(true));
    /// assert_eq!(vec.pop(), Some(true));
    /// assert_eq!(vec.pop(), None);
    /// ```
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

#[cfg(test)]
impl BitVec {
    fn push_unused_word(&mut self) {
        self.buf.push(Word::CLEAR);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use core::iter::repeat_n;
    use std::hash::DefaultHasher;

    #[test]
    fn test_capacity() {
        let vec = BitVec::with_capacity(0);
        assert_eq!(vec.capacity(), 0);

        let vec = BitVec::with_capacity(10);
        assert!(vec.capacity() >= Word::BITS);

        let vec = BitVec::with_capacity(Word::BITS + 1);
        assert!(vec.capacity() >= Word::BITS * 2);
    }

    #[test]
    fn test_len() {
        let vec = bitvec![];
        assert_eq!(vec.len(), 0);

        let vec = bitvec![true, true, false, false];
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_is_empty() {
        let vec = bitvec![];
        assert!(vec.is_empty());

        let vec = bitvec![true, true, false, false];
        assert!(!vec.is_empty());
    }

    #[test]
    fn test_new() {
        let vec = BitVec::new();
        assert_eq!(vec.len, 0);
        assert_eq!(vec.buf, Vec::new());
    }

    #[test]
    fn test_with_capacity() {
        let vec = BitVec::with_capacity(0);
        assert_eq!(vec.len, 0);
        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.buf.capacity(), 0);

        let vec = BitVec::with_capacity(10);
        assert_eq!(vec.len, 0);
        assert!(vec.capacity() >= Word::BITS);
        assert!(vec.buf.capacity() >= 1);

        let vec = BitVec::with_capacity(Word::BITS + 1);
        assert_eq!(vec.len, 0);
        assert!(vec.capacity() >= Word::BITS * 2);
        assert!(vec.buf.capacity() >= 2);
    }

    #[test]
    fn test_reserve() {
        let mut vec = bitvec![true, true, false, false];

        vec.reserve(6);
        assert!(vec.capacity() >= Word::BITS);

        vec.reserve(Word::BITS - vec.len);
        assert!(vec.capacity() >= Word::BITS);

        vec.reserve(Word::BITS);
        assert!(vec.capacity() >= Word::BITS * 2);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut vec = bitvec![true, true, false, false];
        let unchanged = vec.clone();

        vec.reserve(Word::BITS);
        assert!(vec.capacity() >= Word::BITS * 2);

        vec.shrink_to_fit();
        assert!(vec.capacity() >= Word::BITS);
        assert_eq!(vec, unchanged);

        assert_eq!(vec.buf.len(), 1);
        vec.push_unused_word();
        assert_eq!(vec.buf.len(), 2);

        vec.reserve(Word::BITS);
        assert!(vec.capacity() >= Word::BITS * 2);

        vec.shrink_to_fit();
        assert!(vec.capacity() >= Word::BITS);
        assert_eq!(vec, unchanged);
        assert_eq!(vec.buf.len(), 1);
    }

    #[test]
    fn test_shrink_to() {
        let mut vec = bitvec![true, true, false, false];
        let unchanged = vec.clone();

        vec.reserve(Word::BITS * 2);
        assert!(vec.capacity() >= Word::BITS * 3);

        let capacity_unchanged = vec.capacity();
        vec.shrink_to(capacity_unchanged + 1);
        assert_eq!(vec.capacity(), capacity_unchanged);
        assert_eq!(vec, unchanged);

        vec.shrink_to(Word::BITS * 2);
        assert!(vec.capacity() >= Word::BITS * 2);
        assert_eq!(vec, unchanged);

        vec.shrink_to(0);
        assert!(vec.capacity() >= Word::BITS);
        assert_eq!(vec, unchanged);

        assert_eq!(vec.buf.len(), 1);
        vec.push_unused_word();
        assert_eq!(vec.buf.len(), 2);

        vec.reserve(Word::BITS * 2);
        assert!(vec.capacity() >= Word::BITS * 3);

        let capacity_unchanged = vec.capacity();
        vec.shrink_to(capacity_unchanged + 1);
        assert_eq!(vec.capacity(), capacity_unchanged);
        assert_eq!(vec, unchanged);
        assert_eq!(vec.buf.len(), 2);

        vec.shrink_to(Word::BITS * 2);
        assert!(vec.capacity() >= Word::BITS * 2);
        assert_eq!(vec, unchanged);
        assert_eq!(vec.buf.len(), 2);

        vec.shrink_to(0);
        assert!(vec.capacity() >= Word::BITS);
        assert_eq!(vec, unchanged);
        assert_eq!(vec.buf.len(), 1);
    }

    #[test]
    fn test_get() {
        {
            let mut vec = bitvec![true, true, false, false];

            assert_eq!(vec.get(0), Some(true));
            assert_eq!(vec.get(1), Some(true));
            assert_eq!(vec.get(2), Some(false));
            assert_eq!(vec.get(3), Some(false));
            assert_eq!(vec.get(4), None);

            vec.push_unused_word();

            assert_eq!(vec.get(0), Some(true));
            assert_eq!(vec.get(1), Some(true));
            assert_eq!(vec.get(2), Some(false));
            assert_eq!(vec.get(3), Some(false));
            assert_eq!(vec.get(4), None);
        }

        {
            let mut vec = bitvec![true; Word::BITS];

            assert_eq!(vec.get(Word::BITS - 1), Some(true));
            assert_eq!(vec.get(Word::BITS), None);

            vec.push_unused_word();

            assert_eq!(vec.get(Word::BITS - 1), Some(true));
            assert_eq!(vec.get(Word::BITS), None);
        }

        {
            let mut vec = bitvec![true; Word::BITS + 1];

            assert_eq!(vec.get(Word::BITS), Some(true));
            assert_eq!(vec.get(Word::BITS + 1), None);

            vec.push_unused_word();

            assert_eq!(vec.get(Word::BITS), Some(true));
            assert_eq!(vec.get(Word::BITS + 1), None);
        }
    }

    #[test]
    fn test_get_unchecked() {
        let mut vec = bitvec![true, true, false, false];

        unsafe {
            assert!(vec.get_unchecked(0));
            assert!(vec.get_unchecked(1));
            assert!(!vec.get_unchecked(2));
            assert!(!vec.get_unchecked(3));
        }

        vec.push_unused_word();

        unsafe {
            assert!(vec.get_unchecked(0));
            assert!(vec.get_unchecked(1));
            assert!(!vec.get_unchecked(2));
            assert!(!vec.get_unchecked(3));
        }
    }

    #[test]
    fn test_set() {
        {
            let mut vec = bitvec![true, true, false, false];
            let unchanged = vec.clone();

            assert!(vec.set(0, true).is_some());
            assert!(vec.set(1, false).is_some());
            assert!(vec.set(2, true).is_some());
            assert!(vec.set(3, false).is_some());
            assert!(vec.set(4, true).is_none());
            assert_eq!(vec, bitvec![true, false, true, false]);

            let mut vec = unchanged;
            vec.push_unused_word();

            assert!(vec.set(0, true).is_some());
            assert!(vec.set(1, false).is_some());
            assert!(vec.set(2, true).is_some());
            assert!(vec.set(3, false).is_some());
            assert!(vec.set(4, true).is_none());
            assert_eq!(vec, bitvec![true, false, true, false]);
        }

        {
            let mut vec = bitvec![true; Word::BITS];
            let unchanged = vec.clone();

            assert!(vec.set(Word::BITS - 1, false).is_some());
            assert_eq!(vec.get(Word::BITS - 1), Some(false));
            assert!(vec.set(Word::BITS, false).is_none());

            let mut vec = unchanged;
            vec.push_unused_word();

            assert!(vec.set(Word::BITS - 1, false).is_some());
            assert_eq!(vec.get(Word::BITS - 1), Some(false));
            assert!(vec.set(Word::BITS, false).is_none());
        }

        {
            let mut vec = bitvec![true; Word::BITS + 1];
            let unchanged = vec.clone();

            assert!(vec.set(Word::BITS, false).is_some());
            assert_eq!(vec.get(Word::BITS), Some(false));
            assert!(vec.set(Word::BITS + 1, false).is_none());

            let mut vec = unchanged;
            vec.push_unused_word();

            assert!(vec.set(Word::BITS, false).is_some());
            assert_eq!(vec.get(Word::BITS), Some(false));
            assert!(vec.set(Word::BITS + 1, false).is_none());
        }
    }

    #[test]
    fn test_set_unchecked() {
        let mut vec = bitvec![true, true, false, false];
        let unchanged = vec.clone();

        unsafe {
            vec.set_unchecked(2, true);
            vec.set_unchecked(3, true);
        }
        assert_eq!(vec, bitvec![true; 4]);

        let mut vec = unchanged;
        vec.push_unused_word();

        unsafe {
            vec.set_unchecked(2, true);
            vec.set_unchecked(3, true);
        }
        assert_eq!(vec, bitvec![true; 4]);
    }

    #[test]
    fn test_push() {
        {
            let mut vec = bitvec![true, true, false, false];
            let unchanged = vec.clone();

            vec.push(true);
            assert_eq!(vec, bitvec![true, true, false, false, true]);
            vec.push(false);
            assert_eq!(vec, bitvec![true, true, false, false, true, false]);

            let mut vec = unchanged;
            vec.push_unused_word();

            vec.push(true);
            assert_eq!(vec, bitvec![true, true, false, false, true]);
            vec.push(false);
            assert_eq!(vec, bitvec![true, true, false, false, true, false]);
        }

        {
            let mut vec = bitvec![true; Word::BITS];
            let unchanged = vec.clone();

            assert_eq!(vec.buf.len(), 1);
            vec.push(true);
            assert_eq!(vec.len, Word::BITS + 1);
            assert_eq!(vec.get(vec.len - 1), Some(true));
            assert_eq!(vec.buf.len(), 2);
            vec.push(false);
            assert_eq!(vec.len, Word::BITS + 2);
            assert_eq!(vec.get(vec.len - 1), Some(false));
            assert_eq!(vec.buf.len(), 2);

            let mut vec = unchanged;
            vec.push_unused_word();

            assert_eq!(vec.buf.len(), 2);
            vec.push(true);
            assert_eq!(vec.len, Word::BITS + 1);
            assert_eq!(vec.get(vec.len - 1), Some(true));
            assert_eq!(vec.buf.len(), 2);
            vec.push(false);
            assert_eq!(vec.len, Word::BITS + 2);
            assert_eq!(vec.get(vec.len - 1), Some(false));
            assert_eq!(vec.buf.len(), 2);
        }
    }

    #[test]
    fn test_pop() {
        {
            let mut vec = bitvec![true, true, false, false];
            let unchanged = vec.clone();

            assert_eq!(vec.pop(), Some(false));
            assert_eq!(vec, bitvec![true, true, false]);
            assert_eq!(vec.pop(), Some(false));
            assert_eq!(vec, bitvec![true, true]);
            assert_eq!(vec.pop(), Some(true));
            assert_eq!(vec, bitvec![true]);
            assert_eq!(vec.pop(), Some(true));
            assert_eq!(vec, bitvec![]);
            assert_eq!(vec.pop(), None);
            assert_eq!(vec, bitvec![]);

            let mut vec = unchanged;
            vec.push_unused_word();

            assert_eq!(vec.pop(), Some(false));
            assert_eq!(vec, bitvec![true, true, false]);
            assert_eq!(vec.pop(), Some(false));
            assert_eq!(vec, bitvec![true, true]);
            assert_eq!(vec.pop(), Some(true));
            assert_eq!(vec, bitvec![true]);
            assert_eq!(vec.pop(), Some(true));
            assert_eq!(vec, bitvec![]);
            assert_eq!(vec.pop(), None);
            assert_eq!(vec, bitvec![]);
        }

        {
            let mut vec = bitvec![true; Word::BITS + 1];
            let unchanged = vec.clone();

            assert_eq!(vec.buf.len(), 2);
            while vec.pop().is_some() {}
            assert_eq!(vec.len, 0);
            assert_eq!(vec.buf.len(), 2);

            let mut vec = unchanged;
            vec.push_unused_word();

            assert_eq!(vec.buf.len(), 3);
            while vec.pop().is_some() {}
            assert_eq!(vec.len, 0);
            assert_eq!(vec.buf.len(), 3);
        }
    }

    #[test]
    fn test_index() {
        let mut vec = bitvec![true, true, false, false];

        assert!(vec[0]);
        assert!(vec[1]);
        assert!(!vec[2]);
        assert!(!vec[3]);

        vec.push_unused_word();

        assert!(vec[0]);
        assert!(vec[1]);
        assert!(!vec[2]);
        assert!(!vec[3]);
    }

    #[test]
    #[should_panic]
    fn test_index_fails() {
        let vec = bitvec![true, true, false, false];

        let _ = vec[4];
    }

    #[test]
    fn test_clone() {
        let mut vec = bitvec![true, true, false, false];

        let cloned = vec.clone();
        assert_eq!(vec, cloned);

        vec.push_unused_word();

        let cloned = vec.clone();
        assert_eq!(vec, cloned);
        assert_eq!(vec.buf.len(), 2);
        assert_eq!(cloned.buf.len(), 1);
    }

    #[test]
    fn test_extend() {
        let mut vec = bitvec![true, true, false, false];
        let unchanged = vec.clone();

        vec.extend([true; Word::BITS]);
        assert_eq!(vec.len, Word::BITS + 4);
        assert_eq!(vec.get(0), Some(true));
        assert_eq!(vec.get(1), Some(true));
        assert_eq!(vec.get(2), Some(false));
        assert_eq!(vec.get(3), Some(false));
        for index in 4..vec.len {
            assert_eq!(vec.get(index), Some(true));
        }

        let mut vec = unchanged;
        vec.push_unused_word();

        vec.extend([true; Word::BITS]);
        assert_eq!(vec.len, Word::BITS + 4);
        assert_eq!(vec.get(0), Some(true));
        assert_eq!(vec.get(1), Some(true));
        assert_eq!(vec.get(2), Some(false));
        assert_eq!(vec.get(3), Some(false));
        for index in 4..vec.len {
            assert_eq!(vec.get(index), Some(true));
        }
    }

    #[test]
    #[should_panic]
    fn test_extend_fail() {
        let mut vec = bitvec![true, true, false, false];

        vec.extend(repeat_n(true, usize::MAX));
    }

    #[test]
    fn test_hash() {
        fn hash(vec: &BitVec) -> u64 {
            let mut hasher = DefaultHasher::new();
            vec.hash(&mut hasher);
            hasher.finish()
        }

        {
            let lhs = bitvec![true, true, false, false];
            let rhs = bitvec![true; 4];
            let unchanged = rhs.clone();

            let lhs_hash = hash(&lhs);
            let rhs_hash = hash(&rhs);
            assert_ne!(lhs_hash, rhs_hash);

            let mut rhs = unchanged;
            rhs.push_unused_word();

            let lhs_hash = hash(&lhs);
            let rhs_hash = hash(&rhs);
            assert_ne!(lhs_hash, rhs_hash);
        }

        {
            let lhs = bitvec![true, true, false, false];
            let mut rhs = bitvec![true, true, false, false, true];
            let unchanged = rhs.clone();

            let lhs_hash = hash(&lhs);
            let rhs_hash = hash(&rhs);
            assert_ne!(lhs_hash, rhs_hash);
            rhs.pop();
            let rhs_hash = hash(&rhs);
            assert_eq!(lhs_hash, rhs_hash);

            let mut rhs = unchanged;
            rhs.push_unused_word();

            let lhs_hash = hash(&lhs);
            let rhs_hash = hash(&rhs);
            assert_ne!(lhs_hash, rhs_hash);
            rhs.pop();
            let rhs_hash = hash(&rhs);
            assert_eq!(lhs_hash, rhs_hash);
        }

        {
            let lhs = bitvec![true; Word::BITS + 1];
            let mut rhs = lhs.clone();

            let lhs_hash = hash(&lhs);
            let rhs_hash = hash(&rhs);
            assert_eq!(lhs_hash, rhs_hash);
            rhs.push(true).pop();
            let rhs_hash = hash(&rhs);
            assert_eq!(lhs_hash, rhs_hash);

            let mut rhs = lhs.clone();
            rhs.push_unused_word();

            let lhs_hash = hash(&lhs);
            let rhs_hash = hash(&rhs);
            assert_eq!(lhs_hash, rhs_hash);
            rhs.push(true).pop();
            let rhs_hash = hash(&rhs);
            assert_eq!(lhs_hash, rhs_hash);
        }
    }

    #[test]
    fn test_eq() {
        {
            let lhs = bitvec![true, true, false, false];
            let rhs = bitvec![true; 4];
            let unchanged = rhs.clone();

            assert_ne!(lhs, rhs);

            let mut rhs = unchanged;
            rhs.push_unused_word();

            assert_ne!(lhs, rhs);
        }

        {
            let lhs = bitvec![true, true, false, false];
            let mut rhs = bitvec![true, true, false, false, true];
            let unchanged = rhs.clone();

            assert_ne!(lhs, rhs);
            rhs.pop();
            assert_eq!(lhs, rhs);

            let mut rhs = unchanged;
            rhs.push_unused_word();

            assert_ne!(lhs, rhs);
            rhs.pop();
            assert_eq!(lhs, rhs);
        }

        {
            let lhs = bitvec![true; Word::BITS + 1];
            let mut rhs = lhs.clone();

            assert_eq!(lhs, rhs);
            rhs.push(true).pop();
            assert_eq!(lhs, rhs);

            let mut rhs = lhs.clone();
            rhs.push_unused_word();

            assert_eq!(lhs, rhs);
            rhs.push(true).pop();
            assert_eq!(lhs, rhs);
        }
    }
}
