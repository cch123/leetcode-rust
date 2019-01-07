struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        nums1
            .iter()
            .map(|n| {
                set1.insert(n);
            })
            .for_each(drop);
        nums2
            .iter()
            .map(|n| {
                set2.insert(n);
            })
            .for_each(drop);

        let mut res = vec![];
        set2.into_iter()
            .map(|x| {
                if set1.contains(x) {
                    res.push(*x);
                }
            })
            .for_each(drop);
        return res;
    }
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    println!("{:?}", Solution::intersection(nums1, nums2));
}
