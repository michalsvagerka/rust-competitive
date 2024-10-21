//! Immutable tree for range queries on idempotent operations (i.e. min, max).
//!  * Memory O(n log n)
//!  * Build O(n log n)
//!  * Query O(1)

use crate::misc::logceil::logceil;
use std::cmp::{max, min};
use std::marker::PhantomData;
use std::ops::Range;


pub struct ConstTree<T, Op: ConstTreeOp<T>> {
    bit_size: usize,
    length: usize,
    arrays: Vec<Vec<T>>,
    length_to_bits: Vec<usize>,
    op: PhantomData<Op>,
}

pub trait ConstTreeOp<T> {
    fn op(a: T, b: T) -> T;
}

impl<T: Default + Copy, Op: ConstTreeOp<T>> From<Vec<T>> for ConstTree<T, Op> {
    fn from(vec: Vec<T>) -> Self {
        let bit_size = logceil(vec.len());
        let length = vec.len();
        let mut arrays = vec![vec];

        for b in 1..bit_size {
            let prev = &arrays[b - 1];
            let len = 1 << (b - 1);
            arrays.push(
                (0..=(length - 2 * len))
                    .map(|i| Op::op(prev[i], prev[i + len]))
                    .collect::<Vec<_>>(),
            );
        }

        let mut length_to_bits = vec![0; length];
        for i in 2..length {
            length_to_bits[i] = logceil(i) - 1;
        }

        Self {
            bit_size,
            length,
            arrays,
            length_to_bits,
            op: PhantomData,
        }
    }
}

impl<T: Copy, Op: ConstTreeOp<T>> ConstTree<T, Op> {
    fn get(&self, range: Range<usize>) -> T {
        let i = range.start;
        let j = range.end;
        assert!(i < j);
        assert!(i < self.length);
        assert!(j <= self.length);

        let bit_len = self.length_to_bits[j - i - 1];
        Op::op(
            self.arrays[bit_len][i],
            self.arrays[bit_len][j - (1 << bit_len)],
        )
    }

    /*
       void update(const vector<T>&V) {
           A[0] = V;
           for (ui b = 1; b < D; ++b) {
               for (ui i = 0; i + (1<<b) <= N; ++i) {
                   A[b][i] = op(A[b-1][i], A[b-1][i+(1<<(b-1))]);
               }
           }
       }
    */
}

pub struct MinOp {}
pub struct MaxOp {}

impl<T: Ord> ConstTreeOp<T> for MinOp {
    fn op(a: T, b: T) -> T {
        min(a, b)
    }
}

impl<T: Ord> ConstTreeOp<T> for MaxOp {
    fn op(a: T, b: T) -> T {
        max(a, b)
    }
}

#[cfg(test)]
mod test {}