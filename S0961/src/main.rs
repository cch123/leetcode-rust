use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut cnt_map = HashMap::new();
        let must_match_cnt = (a.len() as i32)/2;

        for n in a {
            if let Some(c) = cnt_map.get(&n) {
                cnt_map.insert(n, c+1);
            } else {
                cnt_map.insert(n,1);
            }
        }

        for (k,cnt) in cnt_map {
            if cnt == must_match_cnt {
                return k;
            }
        }
        return 0;
    }
}

fn main() {
    assert_eq!(Solution::repeated_n_times(vec![1,2,3,3]), 3);
    assert_eq!(Solution::repeated_n_times(vec![2,1,2,5,3,2]), 2);
    assert_eq!(Solution::repeated_n_times(vec![5,1,5,2,5,3,5,4]), 5);
}
