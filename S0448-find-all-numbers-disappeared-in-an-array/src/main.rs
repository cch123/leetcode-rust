struct Solution;

use std::collections::HashSet;
impl Solution {
    // 用 set 反而更慢
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        let l = nums.len();
        nums.into_iter()
            .map(|x| {
                set.insert(x);
            })
            .collect::<Vec<_>>();

        let mut v = vec![];

        (0..=l)
            .map(|x| {
                v.push(x as i32);
            })
            .for_each(drop);

        let res = v
            .into_iter()
            .filter(|x| *x != 0 && !set.contains(x))
            .collect();
        return res;
    }

    // 下面这个 16ms
    pub fn find_disappeared_numbers2(nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![];

        (0..=nums.len())
            .map(|x| {
                v.push(x as i32);
            })
            .for_each(drop);

        nums.iter()
            .map(|n| {
                v[*n as usize] = 0;
            })
            .for_each(drop);

        let res = v.into_iter().filter(|x| *x != 0).collect();
        return res;
    }
}

fn main() {
    let v = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", Solution::find_disappeared_numbers(v));
}
