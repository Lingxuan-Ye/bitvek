use super::BitVec;

impl Extend<bool> for BitVec {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = bool>,
    {
        let iter = iter.into_iter();
        let new_len_lower_limit = self
            .len
            .checked_add(iter.size_hint().0)
            .expect("capacity overflow");
        let additional = Self::word_count(new_len_lower_limit) - self.data.len();
        self.data.reserve(additional);
        for value in iter {
            self.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitvec;

    #[test]
    fn test_extend() {
        let mut vec = bitvec![true, false];
        vec.extend([true, false]);
        assert_eq!(vec, bitvec![true, false, true, false]);
    }

    #[test]
    #[should_panic]
    fn test_extend_fail() {
        let mut vec = bitvec![true, false];
        vec.extend(std::iter::repeat(true).take(usize::MAX));
    }
}
