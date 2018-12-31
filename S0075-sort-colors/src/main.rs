struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <=1 {
            return;
        }
        let mut zero_pos = 0usize;
        let mut two_pos = nums.len()-1;
        let mut i =0;
        while i < nums.len() {
            //println!("zero_pos:{}, two_pos:{}, i:{}", zero_pos, two_pos, i);
            if nums[i] == 0 {
                nums.swap(i, zero_pos);
                zero_pos += 1;
                i = zero_pos;
            } else if nums[i] == 2 {
                nums.swap(i,two_pos);
                two_pos -= 1;
                i = zero_pos;
            } else {
                i+=1;
            }
            //println!("after zero_pos:{}, two_pos:{}, i:{}", zero_pos, two_pos, i);
            if zero_pos == two_pos || i > two_pos {
                break;
            }
            //println!("nums {:?}", nums);
        }
    }

    pub fn sort_colors2(nums: &mut Vec<i32>) {
        nums.sort();
    }
}

fn main() {
    let mut nums = vec![1,2,0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
    let mut nums = vec![2];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);

    let mut nums = vec![0,2,1];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
