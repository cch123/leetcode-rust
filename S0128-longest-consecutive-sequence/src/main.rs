struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Bounds {
    pub lower_bound: i32,
    pub upper_bound: i32,
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 记录每个元素对应的上界、下界
        // 如果两个元素相连的话，那么这两个元素对应的上下界一定能连起来
        let mut bounds_m: HashMap<i32, Bounds> = HashMap::new();

        // 记录出现过的元素，如果元素之前出现过，没必要再处理了
        let mut record_m: HashSet<i32> = HashSet::new();
        let mut max_len = 0;
        for num in nums {
            if record_m.contains(&num) {
                continue;
            }

            let left = num - 1;
            let right = num + 1;
            let mut new_bounds = Bounds {
                upper_bound: num,
                lower_bound: num,
            };

            if bounds_m.contains_key(&left) {
                let left_bounds = bounds_m.get(&left).unwrap();
                if left_bounds.upper_bound >= num {
                    continue;
                }

                if left_bounds.upper_bound == left {
                    // 可以合并
                    new_bounds.lower_bound = left_bounds.lower_bound;
                    bounds_m.remove(&left);
                }
            }

            if bounds_m.contains_key(&right) {
                let right_bounds = bounds_m.get(&right).unwrap();
                if right_bounds.lower_bound <= num {
                    continue;
                }

                if right_bounds.lower_bound == right {
                    new_bounds.upper_bound = right_bounds.upper_bound;
                    bounds_m.remove(&right);
                }
            }

            bounds_m.insert(new_bounds.lower_bound, new_bounds.clone());
            bounds_m.insert(new_bounds.upper_bound, new_bounds.clone());
            max_len = max_len.max(new_bounds.upper_bound - new_bounds.lower_bound + 1);
            record_m.insert(num);
        }

        max_len
    }
}

fn main() {
    dbg!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
}
