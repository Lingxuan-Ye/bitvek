use super::BitVec;

impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        let mut lhs = self.data.iter();
        let mut rhs = other.data.iter();

        match (lhs.next_back(), rhs.next_back()) {
            (Some(l), Some(r)) => {
                let lunused = self.unused.value();
                let runused = other.unused.value();
                if lunused != runused {
                    return false;
                }
                if (l >> lunused) != (r >> runused) {
                    return false;
                }
                lhs.eq(rhs)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl Eq for BitVec {}
