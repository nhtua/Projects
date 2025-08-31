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
        let mut e = Float::with_val(precision, 0.0);
        for i in 0..=self.len {
            let term = Float::with_val(precision, 1.0) / factorial(i as u32, precision);
            e += term;
        }
        e
    }
}
