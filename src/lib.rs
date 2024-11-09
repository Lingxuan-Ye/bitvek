const WORD_WIDTH: usize = usize::BITS as usize;

#[derive(Clone, Default)]
pub struct BitVec {
    data: Vec<usize>,
    len: usize,
}
