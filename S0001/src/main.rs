use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h:HashMap<i32,i32>= HashMap::new();
        let mut res = vec![];
        for (idx,i) in nums.iter().enumerate() {
            let resv = target - i;
            match h.get(&resv) {
                Some(v) => {
                    res.push(*v);
                    res.push(idx as i32);
                    return res;
                },
                None => {
                    h.insert(*i,idx as i32);
                    continue;
                },
            }
        }
        println!("{:?}", h);
        return res;
    }
}

fn main() {
    let v = vec![1,2,3,4];
    println!("{:?}",Solution::two_sum(v, 5));
}
