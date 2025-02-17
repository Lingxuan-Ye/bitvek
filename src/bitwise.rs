use super::BitVec;
use std::cmp::min;
use std::iter::zip;

mod and;
mod not;
mod or;
mod xor;

impl BitVec {
    fn bitwise_operation<F>(&self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((&usize, &usize)) -> usize,
    {
        let data = zip(&self.data, &rhs.data).map(op).collect();
        let len = min(self.len, rhs.len);
        Self { data, len }
    }

    fn bitwise_operation_consume_self<F>(self, rhs: &Self, op: F) -> Self
    where
        F: FnMut((usize, &usize)) -> usize,
    {
        let data = zip(self.data, &rhs.data).map(op).collect();
        let len = min(self.len, rhs.len);
        Self { data, len }
    }

    fn bitwise_operation_consume_both<F>(self, rhs: Self, op: F) -> Self
    where
        F: FnMut((usize, usize)) -> usize,
    {
        let data = zip(self.data, rhs.data).map(op).collect();
        let len = min(self.len, rhs.len);
        Self { data, len }
    }
}
