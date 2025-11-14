use crate::BitVec;
use crate::index::Loc;
use core::hash::{Hash, Hasher};

impl Hash for BitVec {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        if self.is_empty() {
            return;
        }

        let last = self.len - 1;
        let loc = Loc::new(last);

        let head = unsafe { self.buf.get_unchecked(..loc.period) };
        head.hash(state);

        let tail = unsafe { self.buf.get_unchecked(loc.period) };
        let unused = loc.complement();
        (tail >> unused).hash(state);

        unused.hash(state);
    }
}
