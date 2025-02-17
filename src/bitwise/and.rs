use super::BitVec;
use std::ops::BitAnd;

impl BitAnd for BitVec {
    type Output = BitVec;

    /// Performs the `&` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitand(self, rhs: BitVec) -> Self::Output {
        self.bitwise_operation_consume_both(rhs, |(left, right)| left & right)
    }
}

impl BitAnd<&BitVec> for BitVec {
    type Output = BitVec;

    /// Performs the `&` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitand(self, rhs: &BitVec) -> Self::Output {
        self.bitwise_operation_consume_self(rhs, |(left, right)| left & right)
    }
}

impl BitAnd<BitVec> for &BitVec {
    type Output = BitVec;

    /// Performs the `&` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitand(self, rhs: BitVec) -> Self::Output {
        rhs & self
    }
}

impl BitAnd for &BitVec {
    type Output = BitVec;

    /// Performs the `&` operation, returning a new `BitVec` with
    /// the length of the shorter input.
    #[inline]
    fn bitand(self, rhs: &BitVec) -> Self::Output {
        self.bitwise_operation(rhs, |(left, right)| left & right)
    }
}
