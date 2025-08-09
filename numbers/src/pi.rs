pub fn find(len: i32) -> f64 {
    let round = len * len * 16;
    let mut i = 1;
    let mut sum: f64 = 0.0;
    while i < round {
        sum += if i % 2 == 0 { -1.0 } else { 1.0 } * aterm(i);
        i += 1;
    }
    sum + 3.0
}
pub fn sprint(len: i32) -> String {
    let pi = find(len);
    pi.to_string().split_at((len + 2) as usize).0.to_string()
}
fn aterm(g: i32) -> f64 {
    4.0 / ((g * 2) as f64 * (g * 2 + 1) as f64 * (g * 2 + 2) as f64)
}
