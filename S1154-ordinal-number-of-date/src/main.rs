fn main() {
    println!("Hello, world!");
}

/*
 * @lc app=leetcode.cn id=1154 lang=rust
 *
 * [1154] 一年中的第几天
 */
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let day_for_normal = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let day_for_leap = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let ymd: Vec<&str> = date.split("-").collect();
        let (y, m, d) = (
            ymd[0].parse::<i32>().unwrap(),
            ymd[1].parse::<i32>().unwrap(),
            ymd[2].parse::<i32>().unwrap(),
        );
        let mut should_use = day_for_normal;
        if Self::is_leap(y) {
            should_use = day_for_leap;
        }
        let mut res = 0;
        (0..m - 1).for_each(|i| {
            res += should_use[i as usize];
        });
        res + d
    }
    fn is_leap(y: i32) -> bool {
        (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
    }
}

