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

#[cfg(test)]
mod tests {
    use crate::bitvec;
    use crate::primitive::Word;

    const LONG: usize = Word::BITS * 2 + 1;
    const SHORT: usize = Word::BITS + 1;

    macro_rules! bitwise_assert {
        ($op:tt, ($input_1:expr, $input_2:expr) => $output:expr) => {
            let vec_1 = bitvec![$input_1; LONG];
            let vec_2 = bitvec![$input_2; SHORT];
            let expected = bitvec![$output; SHORT];
            let unchanged = vec_2.clone();

            assert_eq!(vec_1.clone() $op vec_2.clone(), expected);
            assert_eq!(vec_1.clone() $op &vec_2, expected);
            assert_eq!(&vec_1 $op vec_2.clone(), expected);
            assert_eq!(&vec_1 $op &vec_2, expected);

            let mut vec_2 = unchanged;
            vec_2.push_unused_word();

            assert_eq!(vec_1.clone() $op vec_2.clone(), expected);
            assert_eq!(vec_1.clone() $op &vec_2, expected);
            assert_eq!(&vec_1 $op vec_2.clone(), expected);
            assert_eq!(&vec_1 $op &vec_2, expected);
        };
    }

    #[test]
    fn test_bitand() {
        bitwise_assert!(&, (false, false) => false);
        bitwise_assert!(&, (false, true) => false);
        bitwise_assert!(&, (true, false) => false);
        bitwise_assert!(&, (true, true) => true);
    }

    #[test]
    fn test_bitor() {
        bitwise_assert!(|, (false, false) => false);
        bitwise_assert!(|, (false, true) => true);
        bitwise_assert!(|, (true, false) => true);
        bitwise_assert!(|, (true, true) => true);
    }

    #[test]
    fn test_bitxor() {
        bitwise_assert!(^, (false, false) => false);
        bitwise_assert!(^, (false, true) => true);
        bitwise_assert!(^, (true, false) => true);
        bitwise_assert!(^, (true, true) => false);
    }

    #[test]
    fn test_not() {
        let vec = bitvec![true; SHORT];
        let expected = bitvec![false; SHORT];
        assert_eq!(!&vec, expected);
        assert_eq!(!vec, expected);

        let vec = bitvec![false; LONG];
        let expected = bitvec![true; LONG];
        assert_eq!(!&vec, expected);
        assert_eq!(!vec, expected);
    }
}
