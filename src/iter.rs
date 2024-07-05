//! This module defines several iterators for iterating over a [`BitVec`].
//! You may not need to use them directly.

use super::{BitVec, U3};
use std::iter::FusedIterator;
use std::ops::Range;

impl BitVec {
    /// Returns an iterator over the bits of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::bitvec;
    ///
    /// let vec = bitvec![true, false, true, false];
    /// let mut iter = vec.iter();
    ///
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next_back(), Some(false));
    /// assert_eq!(iter.next_back(), Some(true));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.next_back(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

impl Extend<bool> for BitVec {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = bool>,
    {
        let mut iter = iter.into_iter();

        for _ in 0..self.unused.value() {
            match iter.next() {
                None => return,
                Some(true) => {
                    self.push(true);
                }
                Some(false) => {
                    self.push(false);
                }
            }
        }

        loop {
            let mut byte: u8 = 0;
            for index in 0..8 {
                match iter.next() {
                    None => {
                        let unused = (8 - index) % 8;
                        if unused != 0 {
                            self.data.push(byte);
                        }
                        self.unused = U3(unused);
                        return;
                    }
                    Some(true) => {
                        byte |= 1 << (7 - index);
                    }
                    Some(false) => (),
                }
            }
            self.data.push(byte);
        }
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        let mut vec = BitVec::new();
        vec.extend(iter);
        vec
    }
}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

/// An iterator over the bits of a vector.
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    vec: &'a BitVec,
    range: Range<usize>,
}

impl<'a> Iter<'a> {
    fn new(vec: &'a BitVec) -> Self {
        let range = 0..vec.len();
        Self { vec, range }
    }
}

impl Iterator for Iter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        unsafe { Some(self.vec.get_unchecked(index)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        unsafe { Some(self.vec.get_unchecked(index)) }
    }
}

impl ExactSizeIterator for Iter<'_> {}
impl FusedIterator for Iter<'_> {}

/// An owning iterator over the bits of a vector.
#[derive(Clone, Debug)]
pub struct IntoIter {
    vec: BitVec,
    range: Range<usize>,
}

impl IntoIter {
    fn new(vec: BitVec) -> Self {
        let range = 0..vec.len();
        Self { vec, range }
    }
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        unsafe { Some(self.vec.get_unchecked(index)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        unsafe { Some(self.vec.get_unchecked(index)) }
    }
}

impl ExactSizeIterator for IntoIter {}
impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let vec = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };
        let mut iter = vec.iter();

        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next_back(), Some(true));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_extend() {
        let expected = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };

        let mut vec = BitVec {
            data: vec![0b1000_0000],
            unused: U3(6),
        };
        vec.extend([true, false]);
        assert_eq!(vec, expected);

        let expected = BitVec {
            data: vec![0b1010_1010, 0b1010_1010, 0b1010_0000],
            unused: U3(4),
        };

        let mut vec = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };
        vec.extend([
            true, false, true, false, // first byte
            true, false, true, false, true, false, true, false, // second byte
            true, false, true, false, // final byte
        ]);
        assert_eq!(vec, expected);
    }

    #[test]
    fn test_from_iter() {
        let expected = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };

        let vec = BitVec::from_iter([true, false, true, false]);
        assert_eq!(vec, expected);
    }

    #[test]
    fn test_into_iter() {
        let vec = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };
        let mut iter = vec.into_iter();

        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next_back(), Some(true));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
