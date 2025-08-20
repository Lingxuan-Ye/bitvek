use crate::BitVec;
use core::cmp::min;
use core::iter::zip;

mod and;
mod not;
mod or;
mod xor;

impl BitVec {
    fn bitwise_operation<F>(&self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((&usize, &usize)) -> usize,
    {
        let len = min(self.len, rhs.len);
        let data = zip(&self.data, &rhs.data).map(op).collect();
        Self { len, data }
    }

    fn bitwise_operation_consume_self<F>(self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((usize, &usize)) -> usize,
    {
        let len = min(self.len, rhs.len);
        let data = zip(self.data, &rhs.data).map(op).collect();
        Self { len, data }
    }

    fn bitwise_operation_consume_both<F>(self, rhs: Self, op: F) -> Self
    where
        F: FnMut((usize, usize)) -> usize,
    {
        let len = min(self.len, rhs.len);
        let data = zip(self.data, rhs.data).map(op).collect();
        Self { len, data }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BITS_PER_WORD, bitvec};

    const LONG: usize = BITS_PER_WORD * 2 - 2;
    const SHORT: usize = BITS_PER_WORD - 1;

    macro_rules! bitwise_assert {
        ($op:tt, ($input_1:expr, $input_2:expr) => $output:expr) => {
            let vec_1 = bitvec![$input_1; LONG];
            let vec_2 = bitvec![$input_2; SHORT];
            let expected = bitvec![$output; SHORT];

            assert_eq!(vec_1.clone() $op vec_2.clone(), expected);
            assert_eq!(vec_1.clone() $op &vec_2, expected);
            assert_eq!(&vec_1 $op vec_2.clone(), expected);
            assert_eq!(&vec_1 $op &vec_2, expected);

            assert_eq!(vec_2.clone() $op vec_1.clone(), expected);
            assert_eq!(vec_2.clone() $op &vec_1, expected);
            assert_eq!(&vec_2 $op vec_1.clone(), expected);
            assert_eq!(&vec_2 $op &vec_1, expected);
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
