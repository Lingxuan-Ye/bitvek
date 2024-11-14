use super::{BitVec, BYTES_PER_WORD};

impl BitVec {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let len = bytes.len() * 8;
        let capacity = Self::word_count(len);
        let mut data = Vec::with_capacity(capacity);
        let iter = bytes.chunks_exact(BYTES_PER_WORD);
        let remainder = iter.remainder();
        for chunk in iter {
            let bytes = unsafe { chunk.try_into().unwrap_unchecked() };
            data.push(usize::from_be_bytes(bytes));
        }
        if !remainder.is_empty() {
            let mut last = [0; BYTES_PER_WORD];
            last[..remainder.len()].copy_from_slice(remainder);
            data.push(usize::from_be_bytes(last));
        }
        Self { data, len }
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

impl<const N: usize> From<[u8; N]> for BitVec {
    fn from(value: [u8; N]) -> Self {
        Self::from_bytes(&value)
    }
}

impl From<Vec<u8>> for BitVec {
    fn from(value: Vec<u8>) -> Self {
        Self::from_bytes(&value)
    }
}

impl From<&[u8]> for BitVec {
    fn from(value: &[u8]) -> Self {
        Self::from_bytes(value)
    }
}

impl<const N: usize> From<[bool; N]> for BitVec {
    fn from(value: [bool; N]) -> Self {
        value.into_iter().collect()
    }
}

impl From<Vec<bool>> for BitVec {
    fn from(value: Vec<bool>) -> Self {
        value.into_iter().collect()
    }
}

impl From<&[bool]> for BitVec {
    fn from(value: &[bool]) -> Self {
        value.iter().copied().collect()
    }
}

impl From<BitVec> for Vec<bool> {
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Vec<bool> {
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BITS_PER_WORD;

    #[test]
    fn test_from_bytes() {
        let vec = BitVec::from_bytes(&[0b10101010]);
        assert_eq!(vec.data, vec![0b10101010 << (BITS_PER_WORD - 8)]);
        assert_eq!(vec.len, 8);

        let vec = BitVec::from_bytes(&[0b11110000, 0b00001111]);
        assert_eq!(vec.data, vec![0b11110000_00001111 << (BITS_PER_WORD - 16)]);
        assert_eq!(vec.len, 16);

        let vec = BitVec::from_bytes(&[0b11111111; BYTES_PER_WORD]);
        assert_eq!(vec.data, vec![usize::MAX]);
        assert_eq!(vec.len, BITS_PER_WORD);

        let vec = BitVec::from_bytes(&[0b11111111; BYTES_PER_WORD + 1]);
        assert_eq!(
            vec.data,
            vec![usize::MAX, 0b11111111 << (BITS_PER_WORD - 8)]
        );
        assert_eq!(vec.len, BITS_PER_WORD + 8);
    }

    #[test]
    fn test_from_iter() {
        let vec = BitVec::from_iter([true, false, true, false]);
        assert_eq!(vec.data, vec![0b1010 << (BITS_PER_WORD - 4)]);
        assert_eq!(vec.len, 4);

        let vec = BitVec::from_iter([true; BITS_PER_WORD]);
        assert_eq!(vec.data, vec![usize::MAX]);
        assert_eq!(vec.len, BITS_PER_WORD);

        let vec = BitVec::from_iter([true; BITS_PER_WORD + 1]);
        assert_eq!(vec.data, vec![usize::MAX, 0b1 << (BITS_PER_WORD - 1)]);
        assert_eq!(vec.len, BITS_PER_WORD + 1);
    }
}
