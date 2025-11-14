use crate::index::Loc;
use crate::{BITS_PER_WORD, BitVec};

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

        let lhs = unsafe { self.buf.get_unchecked(..loc.word_index) };
        let rhs = unsafe { other.buf.get_unchecked(..loc.word_index) };
        if lhs != rhs {
            return false;
        }

        let lhs = unsafe { self.buf.get_unchecked(loc.word_index) };
        let rhs = unsafe { other.buf.get_unchecked(loc.word_index) };
        (lhs ^ rhs) >> loc.complement() == 0
    }
}

impl Eq for BitVec {}
