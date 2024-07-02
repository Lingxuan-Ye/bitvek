use super::{BitVec, U3};
use std::iter::FusedIterator;
use std::ops::Range;

impl BitVec {
    /// Returns an iterator over the bits of the vector.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        let mut data = Vec::new();
        let mut iter = iter.into_iter();

        let (last_byte, unused) = 'a: loop {
            let mut byte: u8 = 0;
            for index in 0..8 {
                match iter.next() {
                    None => {
                        let unused = U3((8 - index) % 8);
                        break 'a (byte, unused);
                    }
                    Some(true) => {
                        byte |= 1 << (7 - index);
                    }
                    Some(false) => (),
                }
            }
            data.push(byte);
        };

        if unused != U3(0) {
            data.push(last_byte);
        }

        Self { data, unused }
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
#[derive(Clone)]
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
#[derive(Clone)]
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
