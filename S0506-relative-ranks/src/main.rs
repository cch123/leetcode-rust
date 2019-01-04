struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut sorted= nums.clone();
        sorted.sort_by(|a,b| b.cmp(a));

        for (idx,n) in sorted.iter().enumerate() {
            match idx {
                0 => {
                    map.insert(n, "Gold Medal".to_string());
                },
                1 => {
                    map.insert(n, "Silver Medal".to_string());
                },
                2 => {
                    map.insert(n, "Bronze Medal".to_string());
                },
                _ => {
                    map.insert(n, (idx+1).to_string());
                },
            }
        }
        let mut res = vec![];
        nums.iter().map(|x| {
            let s = map.get(x).unwrap();
            res.push(s.clone());
        }).for_each(drop);
        return res;
    }
}

fn main() {
    println!("Hello, world!");
}
