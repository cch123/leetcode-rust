struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let str = num.to_string();
        let mut max = num;
        let mut b_arr: Vec<u8> = str.bytes().collect();
        let l = b_arr.len();
        for i in 0..l {
            for j in 0..l {
                if i == j {
                    continue;
                }

                b_arr.swap(i,j);
                let cur_val = Solution::bytes_to_int(&b_arr);
                if cur_val > max {
                    max = cur_val;
                }
                b_arr.swap(i,j);
            }
        }
        return max;
    }

    pub fn bytes_to_int(v: &Vec<u8>) -> i32 {
        let mut res: i32 = 0;
        for c in v {
            res = res * 10 + (c - ('0' as u8)) as i32;
        }
        return res;
    }
}

fn main() {
    println!("{}", Solution::maximum_swap(2378));
    println!("{}", Solution::maximum_swap(9972));
    println!("{}", Solution::maximum_swap(98368));
}
