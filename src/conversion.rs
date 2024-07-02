use super::{BitVec, U3};

impl BitVec {
    /// Creates a new [`BitVec`] from a slice of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let vec = BitVec::from_bytes(&[0b1111_0000, 0b0000_1111]);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            data: bytes.to_vec(),
            unused: U3(0),
        }
    }
}

impl<T> From<T> for BitVec
where
    T: AsRef<[bool]>,
{
    fn from(value: T) -> Self {
        value.as_ref().iter().copied().collect()
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

        let vec = BitVec::from_bytes(&[0b1111_0000, 0b0000_1111]);
        assert_eq!(vec, expected);
    }

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
