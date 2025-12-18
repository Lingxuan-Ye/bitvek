use crate::BitVec;
use crate::primitive::Bit;
use core::iter::FusedIterator;
use core::ops::Range;

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
    /// assert_eq!(iter.next_back(), Some(false));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next_back(), Some(false));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.next_back(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        let vec = self;
        let range = 0..vec.len;
        Iter { vec, range }
    }
}

impl IntoIterator for BitVec {
    type Item = Bit;
    type IntoIter = IntoIter;

    #[inline]
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
    type Item = Bit;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        Some(unsafe { self.vec.get_unchecked(index) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        Some(unsafe { self.vec.get_unchecked(index) })
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
    type Item = Bit;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        Some(unsafe { self.vec.get_unchecked(index) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        Some(unsafe { self.vec.get_unchecked(index) })
    }
}

impl ExactSizeIterator for IntoIter {}
impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use crate::bitvec;

    #[test]
    fn test_iter() {
        let vec = bitvec![true, true, false, false];
        let unchanged = vec.clone();

        let mut iter = vec.iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut vec = unchanged;
        vec.push_unused_word();

        let mut iter = vec.iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_into_iter() {
        let vec = bitvec![true, true, false, false];
        let unchanged = vec.clone();

        let mut iter = vec.into_iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut vec = unchanged;
        vec.push_unused_word();

        let mut iter = vec.into_iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next_back(), Some(false));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
