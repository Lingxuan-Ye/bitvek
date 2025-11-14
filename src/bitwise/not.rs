use crate::BitVec;
use core::ops::Not;

impl Not for BitVec {
    type Output = BitVec;

    #[inline]
    fn not(mut self) -> Self::Output {
        let words = self.words_used();
        self.buf
            .iter_mut()
            .take(words)
            .for_each(|word| *word = !*word);
        self
    }
}

impl Not for &BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let len = self.len;
        let buf = self.buf.iter().map(|word| !word).collect();
        Self::Output { len, buf }
    }
}
