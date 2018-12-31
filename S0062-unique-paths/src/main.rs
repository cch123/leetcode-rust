struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // C(m + n- 2) , (n-1)
        let mut res = 1i128;
        for i in 1..=(m+n-2) {
            res = res * (i as i128);
            // 放一个 for 循环里可以防止溢出
            if i <= n-1 {
                res = res / (i as i128);
            } else {
                res = res / ((i-n+1) as i128);
            }
        }
        return res as i32;
    }
}

fn main() {
    println!("{}", Solution::unique_paths(3,2));
    println!("{}", Solution::unique_paths(6,2));
    println!("{}", Solution::unique_paths(36,7));
}
