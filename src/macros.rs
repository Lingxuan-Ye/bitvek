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
