pub use self::iter::{IntoIter, Iter};

mod iter;

const WORD_WIDTH: usize = usize::BITS as usize;

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
        let data_capacity = if capacity == 0 {
            0
        } else {
            (capacity - 1) / WORD_WIDTH + 1
        };
        let data = Vec::with_capacity(data_capacity);
        let len = 0;
        Self { data, len }
    }
}

impl BitVec {
    pub fn capacity(&self) -> usize {
        self.data.capacity() * WORD_WIDTH
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl BitVec {
    pub fn shrink_to_fit(&mut self) -> &mut Self {
        self.data.shrink_to_fit();
        self
    }

    pub fn shrink_to(&mut self, min_capacity: usize) -> &mut Self {
        let data_min_capacity = if min_capacity == 0 {
            0
        } else {
            (min_capacity - 1) / WORD_WIDTH + 1
        };
        self.data.shrink_to(data_min_capacity);
        self
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(self.get_unchecked(index)) }
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        let (div, rem) = (index / WORD_WIDTH, index % WORD_WIDTH);
        let word = unsafe { self.data.get_unchecked(div) };
        let mask = 1 << (WORD_WIDTH - 1 - rem);
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
        let (div, rem) = (index / WORD_WIDTH, index % WORD_WIDTH);
        let word = unsafe { self.data.get_unchecked_mut(div) };
        let mask = 1 << (WORD_WIDTH - 1 - rem);
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
        if self.len != self.data.len() * WORD_WIDTH {
            unsafe { self.set_unchecked(self.len, value) };
        } else if value {
            self.data.push(const { 1 << (WORD_WIDTH - 1) });
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
            if self.len != last * WORD_WIDTH {
                Some(self.get_unchecked(self.len))
            } else {
                self.data.set_len(last);
                Some(*self.data.get_unchecked(last) != 0)
            }
        }
    }
}
