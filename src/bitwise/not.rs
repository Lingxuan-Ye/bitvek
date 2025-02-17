use super::BitVec;
use std::ops::Not;

impl Not for BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let data = self.data.into_iter().map(|word| !word).collect();
        let len = self.len;
        Self::Output { data, len }
    }
}

impl Not for &BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let data = self.data.iter().map(|word| !word).collect();
        let len = self.len;
        Self::Output { data, len }
    }
}
