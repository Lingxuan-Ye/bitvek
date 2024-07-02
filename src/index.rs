use super::BitVec;
use std::ops::Index;

impl Index<usize> for BitVec {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(true) => &true,
            Some(false) => &false,
            None => panic!("index out of bounds"),
        }
    }
}
