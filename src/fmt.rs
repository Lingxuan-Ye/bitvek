use crate::BitVec;
use core::fmt;

impl fmt::Debug for BitVec {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
