use rug::Float;

pub fn sprint(len: u32) -> String {
    let e = find(len);
    e.to_string_radix(10, Some((len + 1) as usize))
}
pub fn find(len: u32) -> Float {
    let precision = (len * 4) as u32; // Each term adds about 1 decimal digit, so 4 times the length for precision
    let mut e = Float::with_val(precision, 0.0);
    for i in 0..=len {
        let term = Float::with_val(precision, 1.0) / factorial(i, precision);
        e += term;
    }
    e
}

fn factorial(n: u32, precision: u32) -> Float {
    if n == 0 {
        Float::with_val(precision, 1.0)
    } else {
        (1..=n).fold(Float::with_val(precision, 1.0), |acc, x| acc * Float::with_val(precision, x))
    }
}
