use crate::BitVec;

mod and;
mod not;
mod or;
mod xor;

impl BitVec {
    fn bitwise_operation<F>(&self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((&usize, &usize)) -> usize,
    {
        let len = self.len.min(rhs.len);
        let words = Self::words_needed(len);
        let buf = self.buf.iter().zip(&rhs.buf).take(words).map(op).collect();
        Self { len, buf }
    }

    fn bitwise_operation_consume_self<F>(self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((usize, &usize)) -> usize,
    {
        let len = self.len.min(rhs.len);
        let words = Self::words_needed(len);
        let buf = self
            .buf
            .into_iter()
            .zip(&rhs.buf)
            .take(words)
            .map(op)
            .collect();
        Self { len, buf }
    }

    fn bitwise_operation_consume_both<F>(self, rhs: Self, op: F) -> Self
    where
        F: FnMut((usize, usize)) -> usize,
    {
        let len = self.len.min(rhs.len);
        let words = Self::words_needed(len);
        let buf = self
            .buf
            .into_iter()
            .zip(rhs.buf)
            .take(words)
            .map(op)
            .collect();
        Self { len, buf }
    }
}
