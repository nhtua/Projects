use rug::Float;

pub trait Finder {
    fn find(&self) -> Float;
    fn sprint(&self) -> String;
}

pub fn factorial(n: u32, precision: u32) -> Float {
    if n == 0 {
        Float::with_val(precision, 1.0)
    } else {
        (1..=n).fold(Float::with_val(precision, 1.0), |acc, x| {
            acc * Float::with_val(precision, x)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::*;
    use rug::Float;

    #[test]
    fn factorial_small() {
        let result = factorial(5, 53);
        let expected_val = Float::with_val(53, 120.0);
        assert_eq!(result, expected_val);
    }

    #[test]
    fn factorial_big() {
        let result = factorial(14, 53);
        let expected_val = Float::with_val(53, 87178291200.0);
        assert_eq!(result, expected_val);
    }
}
