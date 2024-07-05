use super::{BitVec, U3};

impl<const N: usize> From<[bool; N]> for BitVec {
    fn from(value: [bool; N]) -> Self {
        value.into_iter().collect()
    }
}

impl From<Vec<bool>> for BitVec {
    fn from(value: Vec<bool>) -> Self {
        value.into_iter().collect()
    }
}

impl From<&[bool]> for BitVec {
    fn from(value: &[bool]) -> Self {
        value.iter().copied().collect()
    }
}

impl From<BitVec> for Vec<bool> {
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Vec<bool> {
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool_slice() {
        let expected = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };

        let vec = BitVec::from([true, false, true, false]);
        assert_eq!(vec, expected);
    }

    #[test]
    fn test_into_bool_vec() {
        let bits = BitVec::from([true, false, true, false]);

        let bools: Vec<bool> = (&bits).into();
        assert_eq!(bools, vec![true, false, true, false]);

        let bools: Vec<bool> = bits.into();
        assert_eq!(bools, vec![true, false, true, false]);
    }
}
