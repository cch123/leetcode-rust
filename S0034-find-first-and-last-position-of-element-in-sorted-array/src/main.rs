struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1,-1];
        }

        let (mut left_most, mut right_most) = (-1 as i32, -1 as i32);

        // left most
        let (mut low, mut high) = (0 as i32, (nums.len()-1) as i32);
        while low <= high {
            let mid = low + (high - low)/2;
            if nums[mid as usize] == target {
                if left_most == -1 || left_most > mid {
                    left_most = mid;
                }
                high = mid-1;
                continue;
            }
            if nums[mid as usize] < target {
                low = mid+1;continue;
            }
            if nums[mid as usize] > target {
                high = mid-1;continue;
            }
        }

        // right most
        let (mut low, mut high) = (0 as i32, (nums.len()-1) as i32);
        while low <= high {
            let mid = low + (high - low)/2;
            if nums[mid as usize] == target {
                if right_most < mid {
                    right_most = mid;
                    low = mid+1;
                }
                continue;
            }
            if nums[mid as usize] < target {
                low = mid+1;continue;
            }
            if nums[mid as usize] > target {
                high = mid-1;continue;
            }
        }
        return vec![left_most, right_most];
    }
}

fn main() {
    println!("{:?}", Solution::search_range(vec![5,7,7,8,8,10],8));
    println!("{:?}", Solution::search_range(vec![5,7,7,8,8,10],6));
    println!("{:?}", Solution::search_range(vec![],0));
    println!("{:?}", Solution::search_range(vec![1],0));
}
