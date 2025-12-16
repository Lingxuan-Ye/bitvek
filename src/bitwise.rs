use crate::BitVec;
use crate::primitive::Word;

mod and;
mod not;
mod or;
mod xor;

impl BitVec {
    fn bitwise_operation<F>(&self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((&Word, &Word)) -> Word,
    {
        let len = self.len.min(rhs.len);
        let buf_len = len.div_ceil(Word::BITS);
        let buf = self
            .buf
            .iter()
            .zip(&rhs.buf)
            .map(op)
            .take(buf_len)
            .collect();
        Self { len, buf }
    }

    fn bitwise_operation_consume_self<F>(self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((Word, &Word)) -> Word,
    {
        let len = self.len.min(rhs.len);
        let buf_len = len.div_ceil(Word::BITS);
        let buf = self
            .buf
            .into_iter()
            .zip(&rhs.buf)
            .map(op)
            .take(buf_len)
            .collect();
        Self { len, buf }
    }

    fn bitwise_operation_consume_both<F>(self, rhs: Self, op: F) -> Self
    where
        F: FnMut((Word, Word)) -> Word,
    {
        let len = self.len.min(rhs.len);
        let buf_len = len.div_ceil(Word::BITS);
        let buf = self
            .buf
            .into_iter()
            .zip(rhs.buf)
            .map(op)
            .take(buf_len)
            .collect();
        Self { len, buf }
    }
}
