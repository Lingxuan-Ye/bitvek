//! Say, we have a bit vector ---
//!
//! it's nothing better than a [`Vec<bool>`], but ...
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
//!
//! # Memory Efficiency
//!
//! To achieve memory savings, the total number of bits stored must
//! exceed twice the machine word size in bytes, corresponding to 8
//! for 32-bit systems and 16 for 64-bit systems.

#![no_std]

extern crate alloc;

pub use self::iter::{IntoIter, Iter};

use alloc::vec::Vec;
use core::cmp::max;

mod bitwise;
mod convert;
mod eq;
mod extend;
mod fmt;
mod hash;
mod index;
mod iter;
mod macros;

const BITS_PER_BYTE: usize = u8::BITS as usize;
const BITS_PER_WORD: usize = usize::BITS as usize;
const BYTES_PER_WORD: usize = size_of::<usize>();

// As the name suggests, this is a bit vector.
#[derive(Clone, Default)]
pub struct BitVec {
    len: usize,
    data: Vec<usize>,
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
    /// ```
    #[inline]
    pub const fn new() -> Self {
        let len = 0;
        let data = Vec::new();
        Self { len, data }
    }

    /// Creates a new, empty [`BitVec`] with the specified capacity.
    ///
    /// # Notes
    ///
    /// The actual capacity will be rounded up to the nearest multiple of
    /// the machine word size in bits.
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
        let words = Self::words_required(capacity);
        let data = Vec::with_capacity(words);
        Self { len, data }
    }
}

impl BitVec {
    /// Returns the total number of bits the vector can hold
    /// without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let mut vec = BitVec::with_capacity(10);
    /// vec.push(true);
    /// assert!(vec.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.capacity().saturating_mul(BITS_PER_WORD)
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
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
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
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl BitVec {
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
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(self.get_unchecked(index)) }
        }
    }

    /// Returns the bit at the specified index, without performing any
    /// bounds checking.
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
    /// unsafe { assert_eq!(vec.get_unchecked(3), false) };
    /// ```
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        let (div, rem) = (index / BITS_PER_WORD, index % BITS_PER_WORD);
        let word = unsafe { self.data.get_unchecked(div) };
        let mask = 1 << (BITS_PER_WORD - 1 - rem);
        word & mask != 0
    }

    /// Sets the bit at the specified index to the specified value,
    /// if in bounds.
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
    pub fn set(&mut self, index: usize, value: bool) -> Option<&mut Self> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(self.set_unchecked(index, value)) }
        }
    }

    /// Sets the bit at the specified index to the specified value,
    /// without performing any bounds checking.
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
    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) -> &mut Self {
        let (div, rem) = (index / BITS_PER_WORD, index % BITS_PER_WORD);
        let word = unsafe { self.data.get_unchecked_mut(div) };
        let mask = 1 << (BITS_PER_WORD - 1 - rem);
        if value {
            *word |= mask;
        } else {
            *word &= !mask;
        }
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
    pub fn push(&mut self, value: bool) -> &mut Self {
        if self.len == usize::MAX {
            panic!("capacity overflow")
        }
        if self.len % BITS_PER_WORD != 0 {
            // `self.len` as an index is out of bounds and directly
            // violates the safety contract of `Self::set_unchecked`.
            // However, this code is safe due to a full understanding
            // of its internal implementation.
            unsafe {
                self.set_unchecked(self.len, value);
            }
        } else if value {
            self.data.push(const { 1 << (BITS_PER_WORD - 1) });
        } else {
            self.data.push(0);
        }
        self.len += 1;
        self
    }

    /// Removes the last bit from the vector and returns it, or `None` if
    /// the vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let mut vec = bitvec![true, true, false, false];
    /// assert_eq!(vec.pop(), Some(false));
    /// assert_eq!(vec, bitvec![true, true, false]);
    /// ```
    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            return None;
        }
        let last = self.len - 1;
        let value = unsafe { self.get_unchecked(last) };
        if last % BITS_PER_WORD == 0 {
            unsafe {
                self.data.set_len(self.data.len() - 1);
            }
        }
        self.len = last;
        Some(value)
    }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::{BitVec, bitvec};
    ///
    /// let mut vec = BitVec::with_capacity(10);
    /// vec.extend([true, true, false, false]);
    /// assert!(vec.capacity() >= 10);
    /// assert_eq!(vec, bitvec![true, true, false, false]);
    ///
    /// vec.shrink_to_fit();
    /// assert!(vec.capacity() >= 4);
    /// assert_eq!(vec, bitvec![true, true, false, false]);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) -> &mut Self {
        let min_words = Self::words_required(self.len);
        self.data.truncate(min_words);
        self.data.shrink_to_fit();
        self
    }

    /// Shrinks the capacity of the vector with a lower bound.
    ///
    /// If the current capacity is less than the lower limit,
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::{BitVec, bitvec};
    ///
    /// let mut vec = BitVec::with_capacity(10);
    /// vec.extend([true, true, false, false]);
    /// assert!(vec.capacity() >= 10);
    /// assert_eq!(vec, bitvec![true, true, false, false]);
    ///
    /// vec.shrink_to(8);
    /// assert!(vec.capacity() >= 8);
    /// assert_eq!(vec, bitvec![true, true, false, false]);
    ///
    /// vec.shrink_to(0);
    /// assert!(vec.capacity() >= 4);
    /// assert_eq!(vec, bitvec![true, true, false, false]);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) -> &mut Self {
        let min_capacity = max(self.len, min_capacity);
        let min_words = Self::words_required(min_capacity);
        self.data.truncate(min_words);
        self.data.shrink_to_fit();
        self
    }
}

impl BitVec {
    fn words_required(bits: usize) -> usize {
        if bits == 0 {
            0
        } else {
            (bits - 1) / BITS_PER_WORD + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_new() {
        let vec = BitVec::new();
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data, Vec::new());
    }

    #[test]
    fn test_with_capacity() {
        let vec = BitVec::with_capacity(0);
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 0);

        let vec = BitVec::with_capacity(1);
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 1);

        let vec = BitVec::with_capacity(BITS_PER_WORD);
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 1);

        let vec = BitVec::with_capacity(BITS_PER_WORD + 1);
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 2);
    }

    #[test]
    fn test_capacity() {
        let vec = BitVec::with_capacity(0);
        assert_eq!(vec.capacity(), 0);

        let vec = BitVec::with_capacity(1);
        assert_eq!(vec.capacity(), BITS_PER_WORD);

        let vec = BitVec::with_capacity(BITS_PER_WORD);
        assert_eq!(vec.capacity(), BITS_PER_WORD);

        let vec = BitVec::with_capacity(BITS_PER_WORD + 1);
        assert_eq!(vec.capacity(), BITS_PER_WORD * 2);

        // unable to cover (run out of memory)
        // let vec = BitVec {
        //     len: 0,
        //     data: Vec::with_capacity(isize::MAX as usize),
        // };
        // assert_eq!(vec.capacity(), usize::MAX);
    }

    #[test]
    fn test_get() {
        let vec = bitvec![true, true, false, false];
        assert_eq!(vec.get(0), Some(true));
        assert_eq!(vec.get(1), Some(true));
        assert_eq!(vec.get(2), Some(false));
        assert_eq!(vec.get(3), Some(false));
        assert_eq!(vec.get(4), None);

        let vec = bitvec![true; BITS_PER_WORD];
        assert_eq!(vec.get(BITS_PER_WORD - 1), Some(true));
        assert_eq!(vec.get(BITS_PER_WORD), None);

        let vec = bitvec![true; BITS_PER_WORD + 1];
        assert_eq!(vec.get(BITS_PER_WORD), Some(true));
        assert_eq!(vec.get(BITS_PER_WORD + 1), None);
    }

    #[test]
    fn test_set() {
        let mut vec = bitvec![true, true, false, false];
        assert!(vec.set(0, true).is_some());
        assert!(vec.set(1, false).is_some());
        assert!(vec.set(2, true).is_some());
        assert!(vec.set(3, false).is_some());
        assert!(vec.set(4, true).is_none());
        assert_eq!(vec, bitvec![true, false, true, false]);

        let mut vec = bitvec![true; BITS_PER_WORD];
        assert_eq!(vec.get(BITS_PER_WORD - 1), Some(true));
        assert!(vec.set(BITS_PER_WORD - 1, false).is_some());
        assert_eq!(vec.get(BITS_PER_WORD - 1), Some(false));
        assert!(vec.set(BITS_PER_WORD, false).is_none());

        let mut vec = bitvec![true; BITS_PER_WORD + 1];
        assert_eq!(vec.get(BITS_PER_WORD), Some(true));
        assert!(vec.set(BITS_PER_WORD, false).is_some());
        assert_eq!(vec.get(BITS_PER_WORD), Some(false));
        assert!(vec.set(BITS_PER_WORD + 1, false).is_none());
    }

    #[test]
    fn test_push() {
        let mut vec = bitvec![true, true, false, false];
        vec.push(true);
        assert_eq!(vec, bitvec![true, true, false, false, true]);
        vec.push(false);
        assert_eq!(vec, bitvec![true, true, false, false, true, false]);

        let mut vec = bitvec![true; BITS_PER_WORD - 1];
        vec.push(true);
        assert_eq!(vec.len, BITS_PER_WORD);
        assert_eq!(vec.data, vec![usize::MAX]);
        vec.push(false);
        assert_eq!(vec.len, BITS_PER_WORD + 1);
        assert_eq!(vec.data, vec![usize::MAX, 0]);
    }

    #[test]
    fn test_pop() {
        let mut vec = bitvec![true, true, false, false];
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

        let mut vec = bitvec![false; BITS_PER_WORD + 1];
        vec.push(true);
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.pop(), Some(false));

        let mut vec = bitvec![true; BITS_PER_WORD + 1];
        while vec.pop().is_some() {}
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data.len(), 0);
    }
}
