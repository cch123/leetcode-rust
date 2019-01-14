struct Solution;

impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a.clone();
        a.sort_by(|x, y| y.cmp(x));
        //println!("{:?}", a);
        for i in 2..a.len() {
            if Solution::is_valid(a[i], a[i - 1], a[i - 2]) {
                return a[i] + a[i - 1] + a[i - 2];
            }
        }
        0
    }

    pub fn is_valid(a: i32, b: i32, c: i32) -> bool {
        if a + b > c && a + c > b && b + c > a {
            return true;
        }
        return false;
    }
}

fn main() {
    println!("{}", Solution::largest_perimeter(vec![2,1,2]));
}
