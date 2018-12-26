struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_by(|a,b|{
            Solution::cmp(a,b)
        });
        let mut  is_all_zero = true;
        let mut res = "".to_string();
        for num in nums {
            if num > 0 {
                is_all_zero = false;
            }
            res = res + &num.to_string();
        }

        if is_all_zero == true {
            return "0".to_string();
        }

        return res;
    }

    fn cmp(a : &i32, b :&i32) -> Ordering {
        // a 和 b 小于 10 的时候，后面算出来的 pow 有问题，看一下怎么回事
        let mut a_pow = (10.0 as f64).powi((*a as f32).log(10.0) as i32+1) as i128;
        let mut b_pow = (10.0 as f64).powi((*b as f32).log(10.0) as i32+1) as i128;
        let (a,b) = (*a as i128,*b as i128);
        if a == 0 { a_pow = 10};
        if b == 0 { b_pow = 10};
        //println!("{},{},{},{},{},{}",a,b,a_pow,b_pow,a*b_pow+b, b*a_pow+a);
        if a * b_pow + b > b * a_pow + a {
            return Ordering::Less;
        }
        if a * b_pow + b < b * a_pow + a {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

fn main() {
    println!("{}", Solution::largest_number(vec![10,2]));
    println!("{}", Solution::largest_number(vec![3,30,34,5,9]));
    println!("{}", Solution::largest_number(vec![999999998,999999997,999999999]));
    println!("{}", Solution::largest_number(vec![1,2,3,4,5,6,7,8,9,0]));
}
