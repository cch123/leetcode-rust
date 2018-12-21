use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h: HashMap<i32, i32> = HashMap::new();
        for (idx, i) in nums.iter().enumerate() {
            let left = target - i;
            match h.get(&left) {
                Some(v) => return vec![*v, idx as i32],
                None => h.insert(*i, idx as i32),
            };
        }
        return vec![];
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{:?}", Solution::two_sum(v, 5));
}
