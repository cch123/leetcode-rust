struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        return x.powf(n as f64);
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.00,-2));
}
