pub use self::iter::{IntoIter, Iter};

mod convert;
mod eq;
mod extend;
mod fmt;
mod index;
mod iter;
mod macros;

const BITS_PER_WORD: usize = usize::BITS as usize;
const BYTES_PER_WORD: usize = std::mem::size_of::<usize>();

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

    #[must_use]
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

    pub fn push(&mut self, value: bool) -> &mut Self {
        if self.len == usize::MAX {
            panic!("capacity overflow")
        }
        if self.len != self.data.len() * BITS_PER_WORD {
            unsafe { self.set_unchecked(self.len, value) };
        } else if value {
            self.data.push(const { 1 << (BITS_PER_WORD - 1) });
        } else {
            self.data.push(0);
        }
        self.len += 1;
        self
    }

    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            return None;
        }
        let last_bit = self.len - 1;
        let last_word = self.data.len() - 1;
        let value;
        unsafe {
            if last_bit != last_word * BITS_PER_WORD {
                value = self.get_unchecked(last_bit);
            } else {
                value = *self.data.get_unchecked(last_word) != 0;
                self.data.set_len(last_word);
            }
        };
        self.len = last_bit;
        Some(value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec = BitVec::new();
        assert_eq!(vec.data, Vec::new());
        assert_eq!(vec.len, 0);
    }

    #[test]
    fn test_with_capacity() {
        let vec = BitVec::with_capacity(0);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 0);
        assert_eq!(vec.len, 0);

        let vec = BitVec::with_capacity(1);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 1);
        assert_eq!(vec.len, 0);

        let vec = BitVec::with_capacity(BITS_PER_WORD);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 1);
        assert_eq!(vec.len, 0);

        let vec = BitVec::with_capacity(BITS_PER_WORD + 1);
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.data.capacity(), 2);
        assert_eq!(vec.len, 0);
    }

    #[test]
    fn test_capacity() {
        let vec = BitVec::with_capacity(0);
        assert_eq!(vec.capacity(), 0);

        let vec = BitVec::with_capacity(1);
        assert_eq!(vec.capacity(), BITS_PER_WORD);

        let vec = BitVec::with_capacity(BITS_PER_WORD);
        assert_eq!(vec.capacity(), BITS_PER_WORD);

        let vec = BitVec::with_capacity(BITS_PER_WORD + 1);
        assert_eq!(vec.capacity(), BITS_PER_WORD * 2);

        // unable to cover (run out of memory)
        // let vec = BitVec {
        //     data: Vec::with_capacity(isize::MAX as usize / BYTES_PER_WORD),
        //     len: 0,
        // };
        // assert_eq!(vec.capacity(), usize::MAX);
    }

    #[test]
    fn test_get() {
        let vec = bitvec![true, true, false, false];
        assert_eq!(vec.get(0), Some(true));
        assert_eq!(vec.get(1), Some(true));
        assert_eq!(vec.get(2), Some(false));
        assert_eq!(vec.get(3), Some(false));
        assert_eq!(vec.get(4), None);

        let vec = bitvec![true; BITS_PER_WORD];
        assert_eq!(vec.get(BITS_PER_WORD - 1), Some(true));
        assert_eq!(vec.get(BITS_PER_WORD), None);

        let vec = bitvec![true; BITS_PER_WORD + 1];
        assert_eq!(vec.get(BITS_PER_WORD), Some(true));
        assert_eq!(vec.get(BITS_PER_WORD + 1), None);
    }

    #[test]
    fn test_set() {
        let mut vec = bitvec![true, true, false, false];
        assert!(vec.set(0, true).is_some());
        assert!(vec.set(1, false).is_some());
        assert!(vec.set(2, true).is_some());
        assert!(vec.set(3, false).is_some());
        assert!(vec.set(4, true).is_none());
        assert_eq!(vec, bitvec![true, false, true, false]);

        let mut vec = bitvec![true; BITS_PER_WORD];
        assert_eq!(vec.get(BITS_PER_WORD - 1), Some(true));
        assert!(vec.set(BITS_PER_WORD - 1, false).is_some());
        assert_eq!(vec.get(BITS_PER_WORD - 1), Some(false));
        assert!(vec.set(BITS_PER_WORD, false).is_none());

        let mut vec = bitvec![true; BITS_PER_WORD + 1];
        assert_eq!(vec.get(BITS_PER_WORD), Some(true));
        assert!(vec.set(BITS_PER_WORD, false).is_some());
        assert_eq!(vec.get(BITS_PER_WORD), Some(false));
        assert!(vec.set(BITS_PER_WORD + 1, false).is_none());
    }

    #[test]
    fn test_push() {
        let mut vec = bitvec![true, true, false, false];
        vec.push(true);
        assert_eq!(vec, bitvec![true, true, false, false, true]);
        vec.push(false);
        assert_eq!(vec, bitvec![true, true, false, false, true, false]);

        let mut vec = bitvec![true; BITS_PER_WORD - 1];
        vec.push(true);
        assert_eq!(vec.data, vec![usize::MAX]);
        assert_eq!(vec.len, BITS_PER_WORD);
        vec.push(false);
        assert_eq!(vec.data, vec![usize::MAX, 0]);
        assert_eq!(vec.len, BITS_PER_WORD + 1);
    }

    #[test]
    fn test_pop() {
        let mut vec = bitvec![true, true, false, false];
        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec, bitvec![true, true, false]);
        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec, bitvec![true, true]);
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec, bitvec![true]);
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec, bitvec![]);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec, bitvec![]);

        let mut vec = bitvec![true; BITS_PER_WORD + 1];
        while vec.pop().is_some() {}
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.len, 0);
    }
}
