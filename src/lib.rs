#![no_std]

extern crate alloc;

use self::buffer::Buffer;

mod bitwise;
mod buffer;
mod clone;
mod eq;
mod hash;
mod index;

pub type Bit = bool;

type Word = usize;

const BITS_PER_WORD: usize = Word::BITS as usize;

#[derive(Debug, Default)]
pub struct BitVec {
    len: usize,
    buf: Buffer,
}

impl BitVec {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len().saturating_mul(BITS_PER_WORD)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl BitVec {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let len = 0;
        let words = Self::words_needed(capacity);
        let buf = Buffer::allocate(words);
        Self { len, buf }
    }
}

impl BitVec {
    pub fn push(&mut self, value: Bit) -> &mut Self {
        self.reserve(1);
        unsafe {
            self.set_unchecked(self.len, value);
        }
        self.len += 1;
        self
    }

    pub fn pop(&mut self) -> Option<Bit> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(unsafe { self.get_unchecked(self.len) })
        }
    }
}

impl BitVec {
    pub fn reserve(&mut self, additional: usize) -> &mut Self {
        let Some(capacity) = self.len.checked_add(additional) else {
            panic!("capacity overflow");
        };
        let words = Self::words_needed(capacity);
        if words > self.buf.len() {
            let words = self.buf.len().saturating_mul(2).max(words).max(4);
            let mut buf = Buffer::allocate(words);
            unsafe {
                buf.copy_from(&self.buf, self.words_used());
            }
            self.buf = buf;
        }
        self
    }

    pub fn reserve_exact(&mut self, additional: usize) -> &mut Self {
        let Some(capacity) = self.len.checked_add(additional) else {
            panic!("capacity overflow");
        };
        let words = Self::words_needed(capacity);
        if words > self.buf.len() {
            let mut buf = Buffer::allocate(words);
            unsafe {
                buf.copy_from(&self.buf, self.words_used());
            }
            self.buf = buf;
        }
        self
    }

    pub fn shrink_to_fit(&mut self) -> &mut Self {
        let words = self.words_used();
        if words < self.buf.len() {
            let mut buf = Buffer::allocate(words);
            unsafe {
                buf.copy_from(&self.buf, words);
            }
            self.buf = buf;
        }
        self
    }

    pub fn shrink_to(&mut self, capacity: usize) -> &mut Self {
        let capacity = self.len.max(capacity);
        let words = Self::words_needed(capacity);
        if words < self.buf.len() {
            let mut buf = Buffer::allocate(words);
            let count = if self.len == capacity {
                words
            } else {
                self.words_used()
            };
            unsafe {
                buf.copy_from(&self.buf, count);
            }
            self.buf = buf;
        }
        self
    }
}

impl BitVec {
    fn words_needed(bits: usize) -> usize {
        if bits == 0 {
            0
        } else {
            (bits - 1) / BITS_PER_WORD + 1
        }
    }

    fn words_used(&self) -> usize {
        Self::words_needed(self.len)
    }
}
