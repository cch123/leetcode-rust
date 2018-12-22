struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x: i64 = x as i64;
        let mut neg = false;
        if x < 0 {
            x = -x;
            neg = true;
        }

        let mut v = vec![];
        while x > 0 {
            v.push(x % 10);
            x = x / 10;
        }

        let mut res: i64 = 0;
        for i in v {
            res = res * 10 + i as i64;
            if res > i32::max_value() as i64 {
                res = 0;
                break;
            }
        }

        if neg {
            res = -res
        }
        return res as i32;
    }
}

fn main() {
    println!("res : {}", Solution::reverse(111112147));
}
