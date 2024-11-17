use super::BitVec;
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
    /// let vec = bitvec![true, true, false, false];
    /// let mut iter = vec.iter();
    ///
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next_back(), Some(false));
    /// assert_eq!(iter.next_back(), Some(false));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.next_back(), None);
    /// ```
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

/// An iterator over the bits of a vector.
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

/// An owning iterator over the bits of a vector.
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

#[cfg(test)]
mod tests {
    use crate::bitvec;

    #[test]
    fn test_iter() {
        let vec = bitvec![true, false, true, false];
        let mut iter = vec.iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next_back(), Some(true));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let vec = bitvec![true, false, true, false];
        let mut iter = vec.into_iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next_back(), Some(true));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
