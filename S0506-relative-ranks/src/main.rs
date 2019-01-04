struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut sorted= nums.clone();
        sorted.sort_by(|a,b| b.cmp(a));

        let rank_data = vec!["Gold Medal","Silver Medal","Bronze Medal"];
        sorted.iter().enumerate().for_each(|(idx,n)|{
            match idx {
                0|1|2 => {
                    map.insert(n, rank_data[idx].to_string());
                },
                _ => {
                    map.insert(n, (idx+1).to_string());
                },
            }
        });

        nums.iter().map(|x| {
            map.get(x).unwrap().clone()
        }).collect::<Vec<String>>()
    }
}

fn main() {
    println!("Hello, world!");
}
