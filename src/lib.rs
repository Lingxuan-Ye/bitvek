//! A simple bit vector implementation.

mod conversion;
mod fmt;
mod index;
mod iter;

/// Say, we have a bit vector,
/// it's nothing better than a `Vec<bool>`, but ...
/// what if we implement it,
/// and save some poor bits of memory?
#[derive(Clone, Default)]
pub struct BitVec {
    data: Vec<u8>,
    unused: U3,
}

impl BitVec {
    /// Creates a new empty `BitVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let vec = BitVec::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new empty `BitVec` with the specified capacity.
    ///
    /// # Notes
    ///
    /// The final capacity will be `capacity - capacity % 8 + 8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitvek::BitVec;
    ///
    /// let vec = BitVec::with_capacity(10);
    /// assert_eq!(vec.capacity(), 16);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity / 8 + 1),
            unused: U3(0),
        }
    }
}

impl BitVec {
    /// Returns the total number of bits the vector can hold
    /// without reallocating.
    pub fn capacity(&self) -> usize {
        self.data.capacity() * 8
    }

    /// Returns the number of bits in the vector.
    pub fn len(&self) -> usize {
        self.data.len() * 8 - self.unused.value() as usize
    }

    /// Returns `true` if the vector contains no bits.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl BitVec {
    /// Returns the bit at the specified index,
    /// if in bounds.
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(self.get_unchecked(index)) }
    }

    /// Returns the bit at the specified index,
    /// without performing any bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        let (div, rem) = (index / 8, index % 8);
        let byte = unsafe { self.data.get_unchecked(div) };
        let mask = 1 << (7 - rem);
        byte & mask != 0
    }

    /// Sets the bit at the specified index to the specified value,
    /// if in bounds.
    pub fn set(&mut self, index: usize, value: bool) -> Option<&mut Self> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(self.set_unchecked(index, value)) }
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) -> &mut Self {
        let (div, rem) = (index / 8, index % 8);
        let byte = unsafe { self.data.get_unchecked_mut(div) };
        let mask = 1 << (7 - rem);
        if value {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
        self
    }

    /// Appends a bit to the end of the vector.
    pub fn push(&mut self, value: bool) -> Option<&mut Self> {
        if self.unused == U3(0) {
            if self.data.len() == usize::MAX / 8 {
                return None;
            }
            self.data.push(0);
        }
        self.unused.decrement();
        let index = self.len() - 1;
        unsafe { Some(self.set_unchecked(index, value)) }
    }

    /// Removes the last bit from the vector and returns it,
    /// or `None` if the vector is empty.
    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            return None;
        }
        let index = self.len() - 1;
        let value = unsafe { self.get_unchecked(index) };
        self.unused.increment();
        if self.unused == U3(0) {
            self.data.pop();
        }
        Some(value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct U3(u8);

impl U3 {
    fn increment(&mut self) {
        self.0 = (self.0 + 1) % 8;
    }

    fn decrement(&mut self) {
        self.0 = (self.0 + 7) % 8;
    }

    fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(2),
        };

        assert_eq!(vec.get(0), Some(true));
        assert_eq!(vec.get(1), Some(true));
        assert_eq!(vec.get(2), Some(true));
        assert_eq!(vec.get(3), Some(true));
        assert_eq!(vec.get(4), Some(false));
        assert_eq!(vec.get(5), Some(false));
        assert_eq!(vec.get(6), Some(false));
        assert_eq!(vec.get(7), Some(false));
        assert_eq!(vec.get(8), Some(false));
        assert_eq!(vec.get(9), Some(false));
        assert_eq!(vec.get(10), Some(false));
        assert_eq!(vec.get(11), Some(false));
        assert_eq!(vec.get(12), Some(true));
        assert_eq!(vec.get(13), Some(true));
        assert_eq!(vec.get(14), None);
        assert_eq!(vec.get(15), None);
        assert_eq!(vec.get(16), None);
    }

    #[test]
    fn test_set() {
        let mut vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(2),
        };

        assert!(vec.set(0, false).is_some());
        assert!(vec.set(1, false).is_some());
        assert!(vec.set(2, false).is_some());
        assert!(vec.set(3, false).is_some());
        assert!(vec.set(4, true).is_some());
        assert!(vec.set(5, true).is_some());
        assert!(vec.set(6, true).is_some());
        assert!(vec.set(7, true).is_some());
        assert!(vec.set(8, false).is_some());
        assert!(vec.set(9, true).is_some());
        assert!(vec.set(10, false).is_some());
        assert!(vec.set(11, true).is_some());
        assert!(vec.set(12, false).is_some());
        assert!(vec.set(13, true).is_some());
        assert!(vec.set(14, false).is_none());
        assert!(vec.set(15, true).is_none());
        assert!(vec.set(16, false).is_none());
        assert_eq!(vec.data, vec![0b0000_1111, 0b0101_0111]);
    }

    #[test]
    fn test_push() {
        let mut vec = BitVec {
            data: vec![0b1111_0000, 0b0000_1111],
            unused: U3(2),
        };

        assert!(vec.push(false).is_some());
        assert_eq!(vec.data, vec![0b1111_0000, 0b0000_1101]);

        assert!(vec.push(false).is_some());
        assert_eq!(vec.data, vec![0b1111_0000, 0b0000_1100]);

        assert!(vec.push(true).is_some());
        assert_eq!(vec.data, vec![0b1111_0000, 0b0000_1100, 0b1000_0000]);

        assert!(vec.push(true).is_some());
        assert_eq!(vec.data, vec![0b1111_0000, 0b0000_1100, 0b1100_0000]);

        // unable to cover (run out of memory)
        // let mut vec = BitVec {
        //     data: vec![0; usize::MAX / 8],
        //     uninit: U3::build(0).unwrap(),
        // };
        //
        // assert!(vec.push(true).is_none());
    }

    #[test]
    fn test_pop() {
        let mut vec = BitVec {
            data: vec![0b1010_0000],
            unused: U3(5),
        };

        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.data, vec![0b1010_0000]);

        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec.data, vec![0b1010_0000]);

        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.data, vec![]);

        assert_eq!(vec.pop(), None);
        assert_eq!(vec.data, vec![]);
    }
}
