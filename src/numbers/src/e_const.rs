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
        // `to_string_radix` rounds the output. To get a truncated value,
        // we request a higher precision and then truncate the string.
        let mut s = e.to_string_radix(10, Some(self.len + 5));
        let desired_len = self.len + 2; // "2." + `len` decimal digits.
        s.truncate(desired_len);
        s
    }
    fn find(&self) -> Self::Output {
        let precision = (self.len * 4) as u32; // Each term adds about 1 decimal digit, so 4 times the length for precision
        (0..self.len * 2)
            .into_par_iter()
            .map(|i| Float::with_val(precision, 1.0) / factorial(i as u32, precision))
            .reduce(|| Float::with_val(precision, 0), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use crate::e_const::*;

    #[test]
    fn test_e_sprint_short() {
        let euler_const_instance = EulerConst { len: 15 };
        assert_eq!(euler_const_instance.sprint(), "2.718281828459045");
    }
    #[test]
    fn test_e_sprint_long() {
        let euler_const_instance = EulerConst { len: 57 };
        let result = euler_const_instance.sprint();
        assert_eq!(
            result,
            "2.718281828459045235360287471352662497757247093699959574966"
        );
    }
}
