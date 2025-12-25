use crate::BitVec;
use crate::primitive::{Bit, Byte, Word};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr;

impl From<&[Byte]> for BitVec {
    fn from(value: &[Byte]) -> Self {
        let len = value
            .len()
            .checked_mul(Byte::BITS as usize)
            .expect("capacity overflow");
        let buf_len = value.len().div_ceil(Word::BYTES);
        let mut buf = Vec::with_capacity(buf_len);

        let head = value.chunks_exact(Word::BYTES);
        let tail = head.remainder();

        unsafe {
            let mut dst: *mut Word = buf.as_mut_ptr();

            for chunk in head {
                let word = chunk.try_into().unwrap_unchecked();
                let word = Word::from_byte_array(word);
                ptr::write(dst, word);
                dst = dst.add(1);
            }

            if !tail.is_empty() {
                let word = Word::from_byte_slice(tail);
                ptr::write(dst, word);
            }

            buf.set_len(buf_len);
        }

        Self { len, buf }
    }
}

impl<const N: usize> From<[Byte; N]> for BitVec {
    #[inline]
    fn from(value: [Byte; N]) -> Self {
        Self::from(&value[..])
    }
}

impl From<Box<[Byte]>> for BitVec {
    #[inline]
    fn from(value: Box<[Byte]>) -> Self {
        Self::from(&value[..])
    }
}

impl<const N: usize> From<Box<[Byte; N]>> for BitVec {
    #[inline]
    fn from(value: Box<[Byte; N]>) -> Self {
        Self::from(&value[..])
    }
}

impl From<Vec<Byte>> for BitVec {
    #[inline]
    fn from(value: Vec<Byte>) -> Self {
        Self::from(&value[..])
    }
}

impl From<&[Bit]> for BitVec {
    #[inline]
    fn from(value: &[Bit]) -> Self {
        value.iter().copied().collect()
    }
}

impl<const N: usize> From<[Bit; N]> for BitVec {
    #[inline]
    fn from(value: [Bit; N]) -> Self {
        value.into_iter().collect()
    }
}

impl From<Box<[Bit]>> for BitVec {
    #[inline]
    fn from(value: Box<[Bit]>) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> From<Box<[Bit; N]>> for BitVec {
    #[inline]
    fn from(value: Box<[Bit; N]>) -> Self {
        Self::from(value as Box<[Bit]>)
    }
}

impl From<Vec<Bit>> for BitVec {
    #[inline]
    fn from(value: Vec<Bit>) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Box<[Bit]> {
    #[inline]
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}

impl From<BitVec> for Box<[Bit]> {
    #[inline]
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Vec<Bit> {
    #[inline]
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}

impl From<BitVec> for Vec<Bit> {
    #[inline]
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl FromIterator<Bit> for BitVec {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Bit>,
    {
        let iter = iter.into_iter();
        let capacity = iter.size_hint().0;
        let mut vec = BitVec::with_capacity(capacity);
        for value in iter {
            vec.push(value);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitvec;
    use crate::primitive::{Byte, Word};
    use alloc::vec;

    #[test]
    fn test_from_bytes() {
        let vec = BitVec::from([0b11110000]);
        assert_eq!(vec.len, Byte::BITS as usize);
        let mut word = [0; Word::BYTES];
        word[0] = 0b11110000;
        let word = Word::from_byte_array(word);
        assert_eq!(vec.buf, [word]);

        let vec = BitVec::from([0b00000000; Word::BYTES * 2]);
        assert_eq!(vec.len, Word::BITS * 2);
        assert_eq!(vec.buf, [Word::CLEAR; 2]);
    }

    #[test]
    fn test_from_bits() {
        let vec = BitVec::from([true, true, false, false]);
        assert_eq!(vec.len, 4);
        let mut word = [0; Word::BYTES];
        word[0] = 0b11000000;
        let word = Word::from_byte_array(word);
        assert_eq!(vec.buf, [word]);

        let vec = BitVec::from([false; Word::BITS * 2]);
        assert_eq!(vec.len, Word::BITS * 2);
        assert_eq!(vec.buf, [Word::CLEAR; 2]);
    }

    #[test]
    fn test_into_bits() {
        {
            let vec = bitvec![true, true, false, false];
            let expected = vec![true, true, false, false];
            let unchanged = vec.clone();

            let vec = Vec::from(vec);
            assert_eq!(vec, expected);

            let mut vec = unchanged;
            vec.push_unused_word();

            let vec = Vec::from(vec);
            assert_eq!(vec, expected);
        }

        {
            let vec = bitvec![true; Word::BITS + 1];
            let expected = vec![true; Word::BITS + 1];
            let unchanged = vec.clone();

            let vec = Vec::from(vec);
            assert_eq!(vec, expected);

            let mut vec = unchanged;
            vec.push_unused_word();

            let vec = Vec::from(vec);
            assert_eq!(vec, expected);
        }
    }

    #[test]
    fn test_from_iter() {
        let vec = BitVec::from_iter([true, true, false, false]);
        assert_eq!(vec.len, 4);
        let mut word = [0; Word::BYTES];
        word[0] = 0b11000000;
        let word = Word::from_byte_array(word);
        assert_eq!(vec.buf, [word]);

        let vec = BitVec::from_iter([false; Word::BITS * 2]);
        assert_eq!(vec.len, Word::BITS * 2);
        assert_eq!(vec.buf, [Word::CLEAR; 2]);
    }
}
