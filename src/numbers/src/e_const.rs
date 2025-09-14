use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rug::Float;

use crate::processor::{Finder, factorial};

pub struct EulerConst {
    pub len: usize,
}

impl Finder for EulerConst {
    type Output = Float;
    fn sprint(&self) -> String {
        let e = self.find();
        e.to_string_radix(10, Some(self.len + 1))
    }
    fn find(&self) -> Self::Output {
        let precision = (self.len * 4) as u32; // Each term adds about 1 decimal digit, so 4 times the length for precision
        (0..self.len)
            .into_par_iter()
            .map(|i| Float::with_val(precision, 1.0) / factorial(i as u32, precision))
            .reduce(|| Float::with_val(precision, 0), |a, b| a + b)
    }
}
