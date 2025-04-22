use crate::{BITS_PER_WORD, BitVec};
use std::hash::{Hash, Hasher};

impl Hash for BitVec {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        if !self.is_empty() {
            let last = self.data.len() - 1;
            unsafe { self.data.get_unchecked(..last) }.hash(state);
            let last = unsafe { self.data.get_unchecked(last) };
            let unused = self.data.len() * BITS_PER_WORD - self.len;
            (last >> unused).hash(state);
            unused.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitvec;
    use std::hash::DefaultHasher;

    #[test]
    fn test_hash() {
        {
            let lhs = bitvec![true, true, false, false];
            let lhs_hash = {
                let mut hasher = DefaultHasher::new();
                lhs.hash(&mut hasher);
                hasher.finish()
            };

            let rhs = bitvec![true, true, false, false];
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_eq!(lhs_hash, rhs_hash);

            let rhs = bitvec![true, false, true, false];
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_ne!(lhs_hash, rhs_hash);

            let mut rhs = bitvec![true, true, false, false, true];
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_ne!(lhs_hash, rhs_hash);
            rhs.pop();
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_eq!(lhs_hash, rhs_hash);
        }

        {
            let lhs = bitvec![true; BITS_PER_WORD + 1];
            let lhs_hash = {
                let mut hasher = DefaultHasher::new();
                lhs.hash(&mut hasher);
                hasher.finish()
            };

            let rhs = bitvec![true; BITS_PER_WORD + 1];
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_eq!(lhs_hash, rhs_hash);

            let rhs = bitvec![false; BITS_PER_WORD + 1];
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_ne!(lhs_hash, rhs_hash);

            let mut rhs = bitvec![true; BITS_PER_WORD + 2];
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_ne!(lhs_hash, rhs_hash);
            rhs.pop();
            let rhs_hash = {
                let mut hasher = DefaultHasher::new();
                rhs.hash(&mut hasher);
                hasher.finish()
            };
            assert_eq!(lhs_hash, rhs_hash);
        }
    }
}
