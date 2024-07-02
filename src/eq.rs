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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::U3;

    #[test]
    fn test_eq() {
        let expected = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(2),
        };

        let vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(3),
        };
        assert_ne!(vec, expected);

        let vec = BitVec {
            data: vec![0b1111_0000, 0b1100_1111],
            unused: U3(2),
        };
        assert_ne!(vec, expected);

        let vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1111, 0b1111_0000],
            unused: U3(2),
        };
        assert_ne!(vec, expected);

        let vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(2),
        };
        assert_eq!(vec, expected);

        let vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1100],
            unused: U3(2),
        };
        assert_eq!(vec, expected);

        let vec = BitVec::new();
        assert_ne!(vec, expected);
        assert_ne!(expected, vec);

        let expected = BitVec::new();
        assert_eq!(vec, expected);
    }
}
