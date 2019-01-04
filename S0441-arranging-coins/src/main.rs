struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n <= 0 {
            return 0
        }
        let mut res = ((n as f64) * (2.0)).sqrt() as i128;
        if res * (res + 1) / 2 > n as i128 {
            return res as i32 - 1
        }
        res as i32
    }
}

fn main() {
    println!("{}", Solution::arrange_coins(8));
    println!("{}", Solution::arrange_coins(1804289383));
}
