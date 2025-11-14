use crate::buffer::Buffer;
use crate::{BITS_PER_BYTE, BYTES_PER_WORD, Bit, BitVec, Byte, Word};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl BitVec {
    pub fn from_bytes(bytes: &[Byte]) -> Self {
        let len = bytes
            .len()
            .checked_mul(BITS_PER_BYTE)
            .expect("capacity overflow");
        let words = Self::words_needed(len);
        let mut buf = Buffer::allocate(words);

        let iter = bytes.chunks_exact(BYTES_PER_WORD);
        let remainder = iter.remainder();
        for (index, chunk) in iter.enumerate() {
            unsafe {
                let word = chunk.try_into().unwrap_unchecked();
                *buf.get_unchecked_mut(index) = Word::from_be_bytes(word);
            }
        }
        if !remainder.is_empty() {
            unsafe {
                let mut tail = MaybeUninit::<[Byte; _]>::uninit();
                tail.as_mut_ptr()
                    .cast::<Byte>()
                    .copy_from_nonoverlapping(remainder.as_ptr(), remainder.len());
                // SAFETY: `[Byte; _]` has no invalid bit patterns and does not need to drop,
                // so it is safe to assume uninitialized memory as initialized.
                let tail = tail.assume_init();
                *buf.get_unchecked_mut(words - 1) = Word::from_be_bytes(tail);
            }
        }

        Self { len, buf }
    }
}

impl<const N: usize> From<[Byte; N]> for BitVec {
    #[inline]
    fn from(value: [Byte; N]) -> Self {
        Self::from_bytes(&value)
    }
}

impl<const N: usize> From<Box<[Byte; N]>> for BitVec {
    #[inline]
    fn from(value: Box<[Byte; N]>) -> Self {
        Self::from(value as Box<[Byte]>)
    }
}

impl From<Box<[Byte]>> for BitVec {
    #[inline]
    fn from(value: Box<[Byte]>) -> Self {
        Self::from_bytes(&value)
    }
}

impl From<Vec<Byte>> for BitVec {
    #[inline]
    fn from(value: Vec<Byte>) -> Self {
        Self::from_bytes(&value)
    }
}

impl<const N: usize> From<&[Byte; N]> for BitVec {
    #[inline]
    fn from(value: &[Byte; N]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<&[Byte]> for BitVec {
    #[inline]
    fn from(value: &[Byte]) -> Self {
        Self::from_bytes(value)
    }
}

impl<const N: usize> From<[Bit; N]> for BitVec {
    #[inline]
    fn from(value: [Bit; N]) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> From<Box<[Bit; N]>> for BitVec {
    #[inline]
    fn from(value: Box<[Bit; N]>) -> Self {
        Self::from(value as Box<[Bit]>)
    }
}

impl From<Box<[Bit]>> for BitVec {
    #[inline]
    fn from(value: Box<[Bit]>) -> Self {
        value.into_iter().collect()
    }
}

impl From<Vec<Bit>> for BitVec {
    #[inline]
    fn from(value: Vec<Bit>) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> From<&[Bit; N]> for BitVec {
    #[inline]
    fn from(value: &[Bit; N]) -> Self {
        Self::from(&value[..])
    }
}

impl From<&[Bit]> for BitVec {
    #[inline]
    fn from(value: &[Bit]) -> Self {
        value.iter().copied().collect()
    }
}

impl From<BitVec> for Box<[Bit]> {
    #[inline]
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Box<[Bit]> {
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

impl From<&BitVec> for Vec<Bit> {
    #[inline]
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
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
