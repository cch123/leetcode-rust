struct Solution;

use std::collections::HashMap;
impl Solution {
    fn collect_to_maps(nums: Vec<i32>) -> HashMap<i32, i32> {
        let mut map = HashMap::new();
        nums.iter()
            .map(|x| {
                if let Some(cnt) = map.get(x) {
                    map.insert(*x, cnt + 1);
                } else {
                    map.insert(*x, 1);
                }
            })
            .for_each(drop);
        map
    }

    pub fn cintersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let map1 = Solution::collect_to_maps(nums1);
        let map2 = Solution::collect_to_maps(nums2);

        let mut res = vec![];
        map1.iter()
            .map(|(&num, cnt)| {
                let elem_cnt= *map2.get(&num).unwrap_or(&0).min(cnt);

                (0..elem_cnt).map(|_|{
                    res.push(num);
                }).for_each(drop);

            })
            .for_each(drop);

        return res;
    }
}

fn main() {
    let (nums1, nums2) = (vec![1, 2, 2, 1], vec![2, 2]);
    println!("{:?}", Solution::cintersect(nums1, nums2));

    let (nums1, nums2) = (vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    println!("{:?}", Solution::cintersect(nums1, nums2));
}
