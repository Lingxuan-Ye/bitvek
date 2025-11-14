use crate::BitVec;
use crate::index::Loc;

impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        if self.is_empty() {
            return true;
        }

        let last = self.len - 1;
        let loc = Loc::new(last);

        let lhs_head = unsafe { self.buf.get_unchecked(..loc.period) };
        let rhs_head = unsafe { other.buf.get_unchecked(..loc.period) };
        if lhs_head != rhs_head {
            return false;
        }

        let lhs_tail = unsafe { self.buf.get_unchecked(loc.period) };
        let rhs_tail = unsafe { other.buf.get_unchecked(loc.period) };
        (lhs_tail ^ rhs_tail) >> loc.complement() == 0
    }
}

impl Eq for BitVec {}
