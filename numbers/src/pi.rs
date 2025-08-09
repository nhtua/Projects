use rug::Float;

pub fn find(precision: u32) -> Float {
    let xprec = precision + 16;
    let round = precision.pow(2) * 16;
    let mut i: u32 = 1;
    let mut sign: i8 = 1;
    let mut sum: Float = Float::new(xprec);
    while i < round {
        sum += sign * aterm(xprec, i);
        sign *= -1;
        i += 1;
    }
    3 + Float::with_val(xprec, sum)
}
pub fn sprint(len: u32) -> String {
    let pi = find(len);
    pi.to_string_radix(10, Some(len as usize))
        .split_at((len) as usize)
        .0
        .to_string()
}
fn aterm(precision: u32, g: u32) -> Float {
    let four = Float::with_val(precision, 4);
    let g_float = Float::with_val(precision, g);
    let two = Float::with_val(precision, 2);
    let one = Float::with_val(precision, 1);

    let term1 = Float::with_val(precision, &g_float * &two);
    let term2 = Float::with_val(precision, &term1 + &one);
    let term3 = Float::with_val(precision, &term1 + &two);
    four / (term1 * term2 * term3)
}
