use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};
use core::ptr;

pub type Bit = bool;
pub type Byte = u8;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Word(usize);

impl Word {
    pub(crate) const BITS: usize = usize::BITS as usize;
    pub(crate) const BYTES: usize = size_of::<Self>();

    pub(crate) const MSB_SET: Self = Self::mask(0);
    pub(crate) const MSB_CLEAR: Self = Self(0);
    pub(crate) const CLEAR: Self = Self(0);

    /// # Safety
    ///
    /// The invariant `bytes.len() <= Word::BYTES` must hold.
    pub(crate) unsafe fn from_byte_slice(value: &[Byte]) -> Self {
        let mut word = [0; Word::BYTES];
        let src = value.as_ptr();
        let dst = word.as_mut_ptr();
        let count = value.len();
        unsafe {
            ptr::copy_nonoverlapping(src, dst, count);
        }
        Self::from_byte_array(word)
    }

    pub(crate) fn from_byte_array(value: [Byte; Word::BYTES]) -> Self {
        Self(usize::from_be_bytes(value))
    }

    pub(crate) fn to_byte_array(self) -> [Byte; Word::BYTES] {
        self.0.to_be_bytes()
    }

    /// # Notes
    ///
    /// Overflows if `last >= Word::BITS`.
    pub(crate) fn align_last_to_lsb(self, last: usize) -> Self {
        self >> (Self::BITS - 1 - last)
    }

    /// # Notes
    ///
    /// Overflows if `index >= Word::BITS`.
    pub(crate) fn get(&self, index: usize) -> Bit {
        *self & Self::mask(index) != Self(0)
    }

    /// # Notes
    ///
    /// Overflows if `index >= Word::BITS`.
    pub(crate) fn set(&mut self, index: usize, value: Bit) {
        if value {
            *self |= Self::mask(index);
        } else {
            *self &= !Self::mask(index);
        }
    }

    /// # Notes
    ///
    /// Overflows if `index >= Word::BITS`.
    const fn mask(index: usize) -> Self {
        Self(1 << (Self::BITS - 1 - index))
    }
}

impl BitAnd for Word {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Word {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for Word {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Word {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for Word {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Word {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for Word {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Shl<usize> for Word {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<usize> for Word {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}
