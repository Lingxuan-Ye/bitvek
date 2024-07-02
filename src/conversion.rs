use super::BitVec;

impl<T> From<T> for BitVec
where
    T: AsRef<[bool]>,
{
    fn from(value: T) -> Self {
        value.as_ref().iter().copied().collect()
    }
}

impl From<BitVec> for Vec<bool> {
    fn from(value: BitVec) -> Self {
        value.into_iter().collect()
    }
}

impl From<&BitVec> for Vec<bool> {
    fn from(value: &BitVec) -> Self {
        value.iter().collect()
    }
}
