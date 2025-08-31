use crate::processor::Finder;

pub struct Fibonacci {
    pub up_to: usize,
}

impl Finder for Fibonacci {
    type Output = Vec<usize>;
    fn find(&self) -> Self::Output {
        if self.up_to <= 1 {
            return vec![0, 1];
        }
        let mut a: usize = 0;
        let mut b: usize = 1;
        let mut out: Vec<usize> = vec![0, 1];
        while b + a <= self.up_to {
            let temp = b;
            b += a;
            a = temp;
            out.push(b);
        }
        out
    }
    fn sprint(&self) -> String {
        let result = self.find();
        let result_str: Vec<String> = result.iter().map(|&num| num.to_string()).collect();
        result_str.join(" ")
    }
}
