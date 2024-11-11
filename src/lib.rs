pub use self::iter::{IntoIter, Iter};

mod extend;
mod fmt;
mod iter;

const BITS_PER_WORD: usize = usize::BITS as usize;

#[derive(Clone, Default)]
pub struct BitVec {
    data: Vec<usize>,
    len: usize,
}

impl BitVec {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = Self::word_count(capacity);
        let data = Vec::with_capacity(capacity);
        let len = 0;
        Self { data, len }
    }
}

impl BitVec {
    pub fn capacity(&self) -> usize {
        self.data.capacity().saturating_mul(BITS_PER_WORD)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl BitVec {
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(self.get_unchecked(index)) }
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        let (div, rem) = (index / BITS_PER_WORD, index % BITS_PER_WORD);
        let word = unsafe { self.data.get_unchecked(div) };
        let mask = 1 << (BITS_PER_WORD - 1 - rem);
        word & mask != 0
    }

    pub fn set(&mut self, index: usize, value: bool) -> Option<&mut Self> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(self.set_unchecked(index, value)) }
        }
    }

    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) -> &mut Self {
        let (div, rem) = (index / BITS_PER_WORD, index % BITS_PER_WORD);
        let word = unsafe { self.data.get_unchecked_mut(div) };
        let mask = 1 << (BITS_PER_WORD - 1 - rem);
        if value {
            *word |= mask;
        } else {
            *word &= !mask;
        }
        self
    }

    pub fn push(&mut self, value: bool) -> Option<&mut Self> {
        if self.len == usize::MAX {
            return None;
        }
        if self.len != self.data.len() * BITS_PER_WORD {
            unsafe { self.set_unchecked(self.len, value) };
        } else if value {
            self.data.push(const { 1 << (BITS_PER_WORD - 1) });
        } else {
            self.data.push(0);
        }
        self.len += 1;
        Some(self)
    }

    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            return None;
        }
        self.len -= 1;
        let last = self.data.len() - 1;
        unsafe {
            if self.len != last * BITS_PER_WORD {
                Some(self.get_unchecked(self.len))
            } else {
                self.data.set_len(last);
                Some(*self.data.get_unchecked(last) != 0)
            }
        }
    }

    pub fn shrink_to_fit(&mut self) -> &mut Self {
        self.data.shrink_to_fit();
        self
    }

    pub fn shrink_to(&mut self, min_capacity: usize) -> &mut Self {
        let min_capacity = Self::word_count(min_capacity);
        self.data.shrink_to(min_capacity);
        self
    }
}

impl BitVec {
    fn word_count(bits: usize) -> usize {
        if bits == 0 {
            0
        } else {
            (bits - 1) / BITS_PER_WORD + 1
        }
    }
}
