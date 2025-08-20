use crate::BitVec;
use core::ops::Not;

impl Not for BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let len = self.len;
        let data = self.data.into_iter().map(|word| !word).collect();
        Self::Output { len, data }
    }
}

impl Not for &BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let len = self.len;
        let data = self.data.iter().map(|word| !word).collect();
        Self::Output { len, data }
    }
}
