use crate::BitVec;
use core::ops::Not;

impl Not for BitVec {
    type Output = BitVec;

    #[inline]
    fn not(mut self) -> Self::Output {
        let buf_len = self.buf_used();
        self.buf
            .iter_mut()
            .take(buf_len)
            .for_each(|word| *word = !*word);
        self
    }
}

impl Not for &BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let len = self.len;
        let buf_len = self.buf_used();
        let buf = self.buf.iter().map(|word| !*word).take(buf_len).collect();
        BitVec { len, buf }
    }
}
