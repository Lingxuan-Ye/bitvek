use super::BitVec;

impl std::ops::Index<usize> for BitVec {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(true) => &true,
            Some(false) => &false,
            None => panic!("index out of bounds"),
        }
    }
}
