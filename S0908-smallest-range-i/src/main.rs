struct Solution;
impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let (mut max, mut min) = (i32::min_value(), i32::max_value());
        for num in a {
            if max < num {
                max = num;
            }
            if min > num {
                min = num;
            }
        }
        return (max-min - 2*k).max(0);
    }
}

fn main() {
    println!("Hello, world!");
}

/*
impl Solution {
    pub fn smallest_range_i(a: &mut Vec<i32>, k: i32) -> i32 {
        use std::cmp::{self};

        let max = a.iter().cloned().max().unwrap_or(0);
        let min = a.iter().cloned().min().unwrap_or(0);

        cmp::max(max - min - 2 * k, 0)
    }
}
*/