use crate::Word;
use alloc::boxed::Box;
use core::ops::{Deref, DerefMut};
use core::ptr;

#[derive(Debug, Default)]
pub(crate) struct Buffer(Box<[Word]>);

impl Buffer {
    pub(crate) fn allocate(words: usize) -> Self {
        // SAFETY: `Word` has no invalid bit patterns and does not need to drop,
        // so it is safe to assume uninitialized memory as initialized.
        Self(unsafe { Box::new_uninit_slice(words).assume_init() })
    }

    /// # Safety
    ///
    /// Caller must ensure that `self.len().min(src.len()) <= count`.
    pub(crate) unsafe fn copy_from(&mut self, src: &Self, count: usize) {
        let src = src.as_ptr();
        let dst = self.as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping(src, dst, count);
        }
    }
}

impl FromIterator<Word> for Buffer {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Word>,
    {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Buffer {
    type Item = Word;
    type IntoIter = <Box<[usize]> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Buffer {
    type Target = [Word];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
