fn main() {
    println!("Hello, world!");
}

struct Solution;
//https://en.wikipedia.org/wiki/Integer_square_root#Using_only_integer_division
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as f64;
        let (mut a1, mut a2) = (1.0f64, 0.0f64);

        while (a1 - a2).abs() >= 0.1 {
            a2 = a1;
            a1 = (a1 + x / a1) / 2.0;
        }
        a1 as i32
    }
}
