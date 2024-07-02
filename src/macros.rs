/// Creates a new [`BitVec`] from literal.
///
/// [`BitVec`]: crate::BitVec
#[macro_export]
macro_rules! bitvec {
    [$bit:expr; $n:expr] => {
        $crate::BitVec::from([$bit; $n])
    };

    [$($bit:expr),* $(,)?] => {
        $crate::BitVec::from([$($bit,)*])
    };
}
