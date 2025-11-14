use crate::BitVec;
use crate::buffer::Buffer;

impl Clone for BitVec {
    fn clone(&self) -> Self {
        let len = self.len;
        let words = self.words_used();
        let mut buf = Buffer::allocate(words);
        unsafe {
            buf.copy_from(&self.buf, words);
        }
        Self { len, buf }
    }
}
