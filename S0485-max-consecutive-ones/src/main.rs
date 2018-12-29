struct Solution;


impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_counter = 0;
        let mut counter = 0;
        for n in nums {
            if n == 1 {
                counter += 1;
            } else {
                counter = 0;
            }

            if counter > max_counter {
                max_counter = counter;
            }
        }
        return max_counter;
    }
}

fn main() {
    println!("{}", Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]));
}
