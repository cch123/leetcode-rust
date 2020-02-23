struct Solution;

impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        (0..nums.len()).for_each(|i|{
            if nums[i] == i as i32 + 2 && res.len() == 0 {
                res.push(nums[i] - 1);
            }
            if nums[i] == i as i32 + 3 && res.len() == 0 {
                res.push(nums[i] - 2);
                res.push(nums[i] - 1);
            }
            if nums[i] == i as i32 + 3 && res.len() == 1 {
                res.push(nums[i] - 1);
            }
        });

        if res.len() == 0 {
            res.push(nums.len() as i32+1);
            res.push(nums.len() as i32+2);
        }
        if res.len() == 1 {
            res.push(nums.len() as i32 + 2);
        }
        res
    }
}

fn main() {
    dbg!(Solution::missing_two(vec![1]));
    dbg!(Solution::missing_two(vec![2, 3]));
}

