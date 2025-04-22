use crate::BitVec;
use std::ops::BitOr;

impl BitOr for BitVec {
    type Output = BitVec;

    /// Performs the `|` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitor(self, rhs: BitVec) -> Self::Output {
        self.bitwise_operation_consume_both(rhs, |(left, right)| left | right)
    }
}

impl BitOr<&BitVec> for BitVec {
    type Output = BitVec;

    /// Performs the `|` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitor(self, rhs: &BitVec) -> Self::Output {
        self.bitwise_operation_consume_self(rhs, |(left, right)| left | right)
    }
}

impl BitOr<BitVec> for &BitVec {
    type Output = BitVec;

    /// Performs the `|` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitor(self, rhs: BitVec) -> Self::Output {
        rhs | self
    }
}

impl BitOr for &BitVec {
    type Output = BitVec;

    /// Performs the `|` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitor(self, rhs: &BitVec) -> Self::Output {
        self.bitwise_operation(rhs, |(left, right)| left | right)
    }
}
