struct Solution;

impl Solution {
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        if a.len() <= 1 {
            return a.len() as i32;
        }

        let (mut idx, mut max_len) = (1, 1);
        let mut cur_len = 1;
        while idx < (a.len() as i32) {
            if idx %2 != 0 {
                if a[idx as usize] < a[(idx-1) as usize] {
                    cur_len += 1;
                } else {
                    cur_len = 1;
                }
            }
            if idx %2 == 0 {
                if a[idx as usize] > a[(idx-1) as usize] {
                    cur_len += 1;
                } else {
                    cur_len = 1;
                }
            }
            max_len = max_len.max(cur_len);
            idx += 1;
        }

        let mut idx = 1;
        let mut cur_len = 1;
        while idx < (a.len() as i32) {
            if idx %2 == 0 {
                if a[idx as usize] < a[(idx-1) as usize] {
                    cur_len += 1;
                } else {
                    cur_len = 1;
                }
            }
            if idx %2 != 0 {
                if a[idx as usize] > a[(idx-1) as usize] {
                    cur_len += 1;
                } else {
                    cur_len = 1;
                }
            }
            max_len = max_len.max(cur_len);
            idx += 1;
        }
        max_len
    }
}

fn main() {
    println!("{}", Solution::max_turbulence_size(vec![0,1,1,0,1,0,1,1,0,0]));
    println!("{}", Solution::max_turbulence_size(vec![9,4,2,10,7,8,8,1,9]));
}
