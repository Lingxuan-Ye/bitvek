use crate::{BITS_PER_WORD, BitVec};

impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        if self.is_empty() {
            return true;
        }
        let last = self.data.len() - 1;
        if unsafe { self.data.get_unchecked(..last) != other.data.get_unchecked(..last) } {
            return false;
        }
        let xor = unsafe { self.data.get_unchecked(last) ^ other.data.get_unchecked(last) };
        let unused = self.data.len() * BITS_PER_WORD - self.len;
        xor >> unused == 0
    }
}

impl Eq for BitVec {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitvec;

    #[test]
    fn test_eq() {
        let lhs = bitvec![true, false, true, false];

        let rhs = bitvec![true, false, true, false];
        assert_eq!(lhs, rhs);

        let rhs = bitvec![true, true, false, false];
        assert_ne!(lhs, rhs);

        let mut rhs = bitvec![true, false, true, false, true];
        assert_ne!(lhs, rhs);
        rhs.pop();
        assert_ne!(lhs.data, rhs.data);
        assert_eq!(lhs, rhs);

        let lhs = bitvec![true; BITS_PER_WORD + 1];

        let rhs = bitvec![true; BITS_PER_WORD + 1];
        assert_eq!(lhs, rhs);

        let rhs = bitvec![false; BITS_PER_WORD + 1];
        assert_ne!(lhs, rhs);

        let mut rhs = bitvec![true; BITS_PER_WORD + 2];
        assert_ne!(lhs, rhs);
        rhs.pop();
        assert_ne!(lhs.data, rhs.data);
        assert_eq!(lhs, rhs);
    }
}
