struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut cur = digits.len() as i32 - 1;
        let mut carry = 1;
        let mut res = vec![1];
        while cur >= 0 {
            let prev = digits[cur as usize];
            digits[cur as usize] = (prev + carry )% 10;
            if prev + carry >= 10 {
                carry = 1;
            } else {
                carry = 0;
            }
            cur -= 1;
        }
        if carry == 1 {
            res.append(&mut digits);
            return res;
        }
        return digits;
    }
}

fn main() {
    let x = Solution::plus_one(vec![1,2,3]);
    println!("{:?}",x);
    let x = Solution::plus_one(vec![1,2,9]);
    println!("{:?}",x);
    let x = Solution::plus_one(vec![9,9,9]);
    println!("{:?}",x);
}
