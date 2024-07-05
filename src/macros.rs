/// Creates a new [`BitVec`] from literal.
///
/// [`BitVec`]: crate::BitVec
#[macro_export]
macro_rules! bitvec {
    [$elem:expr; $n:expr] => {
        $crate::BitVec::from([$elem; $n])
    };

    [$($elem:expr),* $(,)?] => {
        $crate::BitVec::from([$($elem,)*])
    };
}
