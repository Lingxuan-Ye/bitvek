use crate::{BITS_PER_WORD, Bit, BitVec, Word};
use core::ops::Index;

impl BitVec {
    #[inline]
    pub fn get(&self, index: usize) -> Option<Bit> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> Bit {
        unsafe { Loc::new(index).get_unchecked(&self.buf) }
    }

    #[inline]
    #[must_use]
    pub fn set(&mut self, index: usize, value: Bit) -> Option<&mut Self> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.set_unchecked(index, value) })
        }
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: Bit) -> &mut Self {
        unsafe {
            Loc::new(index).set_unchecked(&mut self.buf, value);
        }
        self
    }
}

impl Index<usize> for BitVec {
    type Output = Bit;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            None => panic!("index out of bounds"),
            Some(false) => &false,
            Some(true) => &true,
        }
    }
}

#[derive(Debug)]
struct Loc {
    word_index: usize,
    bit_offset: usize,
}

impl Loc {
    fn new(index: usize) -> Self {
        Self {
            word_index: index / BITS_PER_WORD,
            bit_offset: index % BITS_PER_WORD,
        }
    }

    fn mask(&self) -> Word {
        1 << (BITS_PER_WORD - 1 - self.bit_offset)
    }

    unsafe fn get_unchecked(self, buf: &[Word]) -> Bit {
        let word = unsafe { buf.get_unchecked(self.word_index) };
        let mask = self.mask();
        word & mask != 0
    }

    unsafe fn set_unchecked(self, buf: &mut [Word], value: Bit) {
        let word = unsafe { buf.get_unchecked_mut(self.word_index) };
        let mask = self.mask();
        if value {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
}
