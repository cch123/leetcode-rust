struct Solution;
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut res = a.iter().map(|x|(*x)*(*x)).collect::<Vec<_>>();
        res.sort();
        res
    }
}

fn main() {
    println!("Hello, world!");
}
