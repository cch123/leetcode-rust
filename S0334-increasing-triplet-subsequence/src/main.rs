struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut low, mut high) = (i32::max_value(), i32::max_value());
        for num in nums {
            if num <= low {
                low = num;
            } else if num <= high {
                high = num;
            } else {
                return true
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
