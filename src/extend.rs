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
            self.push(value).expect("capacity overflow");
        }
    }
}
