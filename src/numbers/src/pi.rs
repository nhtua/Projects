use rayon::prelude::*;
use rug::Float;
use rug::ops::Pow;

use crate::processor::{Finder, factorial};

pub struct Pi {
    pub len: usize,
}

impl Finder for Pi {
    type Output = Float;
    /// Module interface
    /// Return big value as a string
    fn sprint(&self) -> String {
        let pi = self.find();
        pi.to_string_radix(10, Some(self.len + 1))
    }

    /// Module interface
    /// Note that `len` here is the number of terms that calculates sigma, not the len of precision
    fn find(&self) -> Self::Output {
        chudnovsky(self.len as u32)
    }
}

fn chudnovsky(len: u32) -> Float {
    //rug::Float(precision) **precision**  is a fixed number to determine the length of an arbitrary number.
    //So we have to make precision far larger then the number_of_term to it have enough room to
    //contains the output. However each term gives about 14 digits after decimal point.
    //Then not necessarily len==term. By practical experiment, I found number_of_term could be
    let number_of_term: u32 = len / 4;
    let precision: u32 = number_of_term * 14;
    Float::with_val(precision, 1)
        / (Float::with_val(precision, 12) * sigma(number_of_term, precision))
}

fn sigma(number_of_term: u32, precision: u32) -> Float {
    (0..number_of_term)
        .into_par_iter()
        .map(|k| {
            let sign_val = if k % 2 == 0 { 1.0f64 } else { -1.0f64 };
            let up = Float::with_val(precision, sign_val)
                * factorial(6 * k, precision)
                * Float::with_val(precision, 545140134 * k as i64 + 13591409);
            let down_fact_3k = factorial(3 * k, precision);
            let down_fact_k_pow3 = factorial(k, precision).pow(Float::with_val(precision, 3.0));
            let down_const_pow = Float::with_val(precision, 640320.0f64)
                .pow(Float::with_val(precision, (3 * k) as f64 + 1.5));
            let down: Float = down_fact_3k * down_fact_k_pow3 * down_const_pow;
            up / down
        })
        .reduce(|| Float::with_val(precision, 0), |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use crate::pi::*;
    use core::f64;
    use rug::Float;

    #[test]
    fn calculate_small_pi() {
        // find(), chudnovsky() and sigma() are actually parts of
        // the same function (Chudnovsky equation)
        // Just test find() is good is enough.
        let p = Pi { len: 10 };
        let mut result = p.find();
        result.set_prec(12);
        let expectation = Float::with_val(12, f64::consts::PI);
        assert_eq!(result, expectation);
    }
    #[test]
    fn calculate_big_pi() {
        // sprint(), find(), chudnovsky() and sigma() are actually parts of
        // the same function (Chudnovsky equation)
        // Just test sprint() is good is enough.
        let p = Pi { len: 120 };
        let result = p.sprint(); // excessive 1 is because of the beginning with 3.
        let expectation = "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647";
        assert_eq!(result, expectation);
    }
}
