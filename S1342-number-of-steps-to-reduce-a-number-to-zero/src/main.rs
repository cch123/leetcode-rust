fn main() {
    dbg!(Solution::number_of_steps(14));
}

struct Solution;

impl Solution {
    pub fn number_of_steps (num: i32) -> i32 {
        (num.count_ones() * 2 - 1 + num.count_zeros() - num.leading_zeros()) as i32
    }
}