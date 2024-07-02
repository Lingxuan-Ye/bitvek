use super::BitVec;
use std::fmt::{Debug, Formatter, Result};

impl Debug for BitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
