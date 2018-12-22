struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut xx: i64 = x as i64;
        let mut neg = false;
        if x < 0 {
            neg = true;
            xx = -xx;
        }
        unsafe {
            let mut xx_str = xx.to_string();
            let s = xx_str.as_bytes_mut();
            let mut v = vec![];
            let mut idx: usize = s.len() - 1;
            loop {
                v.push(s[idx]);
                if idx == 0 {
                    break;
                }
                // usize 减到负数会 panic， 注意
                idx -= 1;
            }

            let res: i32 = String::from_utf8(v)
                .unwrap()
                .parse()
                .unwrap_or_else(|_| return 0);

            if !neg {
                return res as i32;
            } else {
                return -res as i32;
            }
        }
    }
}

fn main() {
    println!("res : {}", Solution::reverse(-2147483648));
}
