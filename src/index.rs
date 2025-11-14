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
pub(crate) struct Loc {
    pub(crate) period: usize,
    pub(crate) offset: usize,
}

impl Loc {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            period: index / BITS_PER_WORD,
            offset: index % BITS_PER_WORD,
        }
    }

    pub(crate) fn complement(&self) -> usize {
        BITS_PER_WORD - 1 - self.offset
    }

    pub(crate) fn mask(&self) -> Word {
        1 << self.complement()
    }

    pub(crate) unsafe fn get_unchecked(self, buf: &[Word]) -> Bit {
        let word = unsafe { buf.get_unchecked(self.period) };
        let mask = self.mask();
        word & mask != 0
    }

    pub(crate) unsafe fn set_unchecked(self, buf: &mut [Word], value: Bit) {
        let word = unsafe { buf.get_unchecked_mut(self.period) };
        let mask = self.mask();
        if value {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
}
