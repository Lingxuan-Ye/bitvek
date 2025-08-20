use crate::{BITS_PER_BYTE, BYTES_PER_WORD, BitVec};
use alloc::boxed::Box;
use alloc::vec::Vec;

impl BitVec {
    /// Creates a new `BitVec` from the given bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::{bitvec, BitVec};
    ///
    /// let vec = BitVec::from_bytes(&[0b11110000, 0b00001111]);
    /// assert_eq!(vec, bitvec![
    ///     true, true, true, true, false, false, false, false,
    ///     false, false, false, false, true, true, true, true,
    /// ]);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let len = bytes.len() * BITS_PER_BYTE;
        let words = Self::words_required(len);
        let mut data = Vec::with_capacity(words);
        let iter = bytes.chunks_exact(BYTES_PER_WORD);
        let remainder = iter.remainder();
        for chunk in iter {
            let bytes = unsafe { chunk.try_into().unwrap_unchecked() };
            data.push(usize::from_be_bytes(bytes));
        }
        if !remainder.is_empty() {
            let mut last = [0; BYTES_PER_WORD];
            unsafe {
                last.get_unchecked_mut(..remainder.len())
                    .copy_from_slice(remainder);
            }
            data.push(usize::from_be_bytes(last));
        }
        Self { len, data }
    }
}

impl<const N: usize> From<[u8; N]> for BitVec {
    #[inline]
    fn from(value: [u8; N]) -> Self {
        Self::from_bytes(&value)
    }
}

impl<const N: usize> From<Box<[u8; N]>> for BitVec {
    #[inline]
    fn from(value: Box<[u8; N]>) -> Self {
        Self::from(value as Box<[u8]>)
    }
}

impl From<Box<[u8]>> for BitVec {
    #[inline]
    fn from(value: Box<[u8]>) -> Self {
        Self::from_bytes(&value)
    }
}

impl From<Vec<u8>> for BitVec {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self::from_bytes(&value)
    }
}

impl<const N: usize> From<&[u8; N]> for BitVec {
    #[inline]
    fn from(value: &[u8; N]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<&[u8]> for BitVec {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::from_bytes(value)
    }
}

impl<const N: usize> From<[bool; N]> for BitVec {
    #[inline]
    fn from(value: [bool; N]) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> From<Box<[bool; N]>> for BitVec {
    #[inline]
    fn from(value: Box<[bool; N]>) -> Self {
        Self::from(value as Box<[bool]>)
    }
}

impl From<Box<[bool]>> for BitVec {
    #[inline]
    fn from(value: Box<[bool]>) -> Self {
        value.into_iter().collect()
    }
}

impl From<Vec<bool>> for BitVec {
    #[inline]
    fn from(value: Vec<bool>) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> From<&[bool; N]> for BitVec {
    #[inline]
    fn from(value: &[bool; N]) -> Self {
        Self::from(&value[..])
    }
}

impl From<&[bool]> for BitVec {
    #[inline]
    fn from(value: &[bool]) -> Self {
        value.iter().copied().collect()
    }
}

impl From<BitVec> for Box<[bool]> {
    #[inline]
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Box<[bool]> {
    #[inline]
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}

impl From<BitVec> for Vec<bool> {
    #[inline]
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Vec<bool> {
    #[inline]
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        let iter = iter.into_iter();
        let mut vec = BitVec::with_capacity(iter.size_hint().0);
        for value in iter {
            vec.push(value);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BITS_PER_WORD;
    use alloc::vec;

    #[test]
    fn test_from_bytes() {
        let vec = BitVec::from_bytes(&[0b10101010]);
        assert_eq!(vec.len, 8);
        assert_eq!(vec.data, vec![0b10101010 << (BITS_PER_WORD - 8)]);

        let vec = BitVec::from_bytes(&[0b11110000, 0b00001111]);
        assert_eq!(vec.len, 16);
        assert_eq!(vec.data, vec![0b11110000_00001111 << (BITS_PER_WORD - 16)]);

        let vec = BitVec::from_bytes(&[0b11111111; BYTES_PER_WORD]);
        assert_eq!(vec.len, BITS_PER_WORD);
        assert_eq!(vec.data, vec![usize::MAX]);

        let vec = BitVec::from_bytes(&[0b11111111; BYTES_PER_WORD + 1]);
        assert_eq!(vec.len, BITS_PER_WORD + 8);
        assert_eq!(
            vec.data,
            vec![usize::MAX, 0b11111111 << (BITS_PER_WORD - 8)]
        );
    }

    #[test]
    fn test_from_iter() {
        let vec = BitVec::from_iter([true, false, true, false]);
        assert_eq!(vec.len, 4);
        assert_eq!(vec.data, vec![0b1010 << (BITS_PER_WORD - 4)]);

        let vec = BitVec::from_iter([true; BITS_PER_WORD]);
        assert_eq!(vec.len, BITS_PER_WORD);
        assert_eq!(vec.data, vec![usize::MAX]);

        let vec = BitVec::from_iter([true; BITS_PER_WORD + 1]);
        assert_eq!(vec.len, BITS_PER_WORD + 1);
        assert_eq!(vec.data, vec![usize::MAX, 0b1 << (BITS_PER_WORD - 1)]);
    }
}
