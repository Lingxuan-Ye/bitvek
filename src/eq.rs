use super::{BitVec, BITS_PER_WORD};

impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        if self.is_empty() {
            return true;
        }
        let last = self.data.len() - 1;
        if self.data[..last] != other.data[..last] {
            return false;
        }
        (self.data[last] ^ other.data[last]) >> (self.data.len() * BITS_PER_WORD - self.len) == 0
    }
}

impl Eq for BitVec {}
