use crate::{Bit, BitVec};

impl Extend<Bit> for BitVec {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Bit>,
    {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for value in iter {
            self.push(value);
        }
    }
}
