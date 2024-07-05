use super::{BitVec, U3};

impl<const N: usize> From<[u8; N]> for BitVec {
    fn from(value: [u8; N]) -> Self {
        Self {
            data: value.to_vec(),
            unused: U3(0),
        }
    }
}

impl From<Vec<u8>> for BitVec {
    fn from(value: Vec<u8>) -> Self {
        Self {
            data: value,
            unused: U3(0),
        }
    }
}

impl From<&[u8]> for BitVec {
    fn from(value: &[u8]) -> Self {
        Self {
            data: value.to_vec(),
            unused: U3(0),
        }
    }
}

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
    fn test_from_bytes() {
        let expected = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(0),
        };
        let bytes: [u8; 2] = [0b1111_0000, 0b0000_1111];

        let bits = BitVec::from(&bytes[..]);
        assert_eq!(bits, expected);

        let bits = BitVec::from(bytes);
        assert_eq!(bits, expected);

        let bits = BitVec::from(bytes.to_vec());
        assert_eq!(bits, expected);
    }

    #[test]
    fn test_from_bools() {
        let expected = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };
        let bools = [true, false, true, false];

        let bits = BitVec::from(&bools[..]);
        assert_eq!(bits, expected);

        let bits = BitVec::from(bools);
        assert_eq!(bits, expected);

        let bits = BitVec::from(bools.to_vec());
        assert_eq!(bits, expected);
    }

    #[test]
    fn test_into_bools() {
        let expected = vec![true, false, true, false];
        let bits = BitVec {
            data: vec![0b1010_0000],
            unused: U3(4),
        };

        let bools: Vec<bool> = (&bits).into();
        assert_eq!(bools, expected);

        let bools: Vec<bool> = bits.into();
        assert_eq!(bools, expected);
    }
}
