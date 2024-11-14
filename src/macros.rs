#[macro_export]
macro_rules! bitvec {
    [] => {
        $crate::BitVec::new()
    };

    [$elem:expr; $n:expr] => {
        $crate::BitVec::from(::std::vec![$elem; $n])
    };

    [$($elem:expr),+ $(,)?] => {
        $crate::BitVec::from([$($elem,)+])
    };
}

#[cfg(test)]
mod tests {
    use crate::{BITS_PER_WORD, BYTES_PER_WORD};

    #[test]
    fn test_bitvec() {
        let vec = bitvec![];
        assert_eq!(vec.data, Vec::new());
        assert_eq!(vec.len, 0);

        let vec = bitvec![false; 2];
        assert_eq!(vec.data, vec![0]);
        assert_eq!(vec.len, 2);

        let vec = bitvec![true; 2];
        assert_eq!(vec.data, vec![0b11 << (BITS_PER_WORD - 2)]);
        assert_eq!(vec.len, 2);

        let vec = bitvec![true, false, true, false];
        assert_eq!(vec.data, vec![0b1010 << (BITS_PER_WORD - 4)]);
        assert_eq!(vec.len, 4);

        let vec = bitvec![true, true, false, false];
        assert_eq!(vec.data, vec![0b1100 << (BITS_PER_WORD - 4)]);
        assert_eq!(vec.len, 4);

        let vec = bitvec![true; BITS_PER_WORD + 1];
        assert_eq!(vec.data, vec![usize::MAX, 0b1 << (BITS_PER_WORD - 1)]);
        assert_eq!(vec.len, BITS_PER_WORD + 1);

        let vec = bitvec![0b10101010; 2];
        assert_eq!(vec.data, vec![0b10101010_10101010 << (BITS_PER_WORD - 16)]);
        assert_eq!(vec.len, 16);

        let vec = bitvec![0b11110000, 0b00001111];
        assert_eq!(vec.data, vec![0b11110000_00001111 << (BITS_PER_WORD - 16)]);
        assert_eq!(vec.len, 16);

        let vec = bitvec![0b11111111; BYTES_PER_WORD + 1];
        assert_eq!(
            vec.data,
            vec![usize::MAX, 0b11111111 << (BITS_PER_WORD - 8)]
        );
        assert_eq!(vec.len, (BYTES_PER_WORD + 1) * 8);
    }
}
