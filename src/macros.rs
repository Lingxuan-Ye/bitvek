/// Creates a new [`BitVec`] from a literal.
///
/// # Examples
///
/// ```
/// use bitvek::bitvec;
///
/// let foo = bitvec![];
/// let bar = bitvec![true; 2];
/// let baz = bitvec![true, true, false, false];
/// let qux = bitvec![0b11110000, 0b00001111];
/// ```
///
/// [`BitVec`]: crate::BitVec
#[macro_export]
macro_rules! bitvec {
    [] => {
        $crate::BitVec::new()
    };

    [$elem:expr; $n:expr] => {
        $crate::BitVec::from([$elem; $n])
    };

    [$($elem:expr),+ $(,)?] => {
        $crate::BitVec::from([$($elem,)+])
    };
}
