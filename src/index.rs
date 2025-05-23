use crate::BitVec;
use core::ops::Index;

impl Index<usize> for BitVec {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(true) => &true,
            Some(false) => &false,
            None => panic!("index out of bounds"),
        }
    }
}
