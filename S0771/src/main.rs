use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let j = j.as_bytes();
        let s = s.as_bytes();
        let mut cnt = 0;
        let mut diamond_set = HashSet::new();
        for c in j {
            diamond_set.insert(c);
        }
        for c in s {
            if diamond_set.contains(c) {
                cnt += 1;
            }
        }
        return cnt;
    }
}

fn main() {
    println!("{}", Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()));
    println!("{}", Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()));

}
