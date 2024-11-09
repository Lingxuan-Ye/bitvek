use super::BitVec;
use std::iter::FusedIterator;
use std::ops::Range;

impl BitVec {
    pub fn iter(&self) -> Iter<'_> {
        let vec = self;
        let range = 0..vec.len;
        Iter { vec, range }
    }
}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let vec = self;
        let range = 0..vec.len;
        IntoIter { vec, range }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    vec: &'a BitVec,
    range: Range<usize>,
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

#[derive(Clone, Debug)]
pub struct IntoIter {
    vec: BitVec,
    range: Range<usize>,
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
