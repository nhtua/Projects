use rayon::prelude::*;
use rug::Float;
use rug::ops::Pow;

pub fn sprint(len: u32) -> String {
    let n_term = len / 8 + 1;
    let pi = chudnovsky(n_term, len);
    pi.to_string_radix(10, Some(len as usize))
        .split_at((len) as usize)
        .0
        .to_string()
}

fn chudnovsky(number_of_term: u32, precision: u32) -> Float {
    Float::with_val(precision, 1)
        / (Float::with_val(precision, 12) * sigma(number_of_term, precision))
}
fn sigma(number_of_term: u32, precision: u32) -> Float {
    let round: u32 = number_of_term * 14;
    (0..round)
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
fn factorial(n: u32, precision: u32) -> Float {
    (2..=n).fold(Float::with_val(precision, 1), |acc, x| {
        acc * Float::with_val(precision, x)
    })
}
