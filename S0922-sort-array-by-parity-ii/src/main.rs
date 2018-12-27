struct Solution;
impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        if a.len() == 0 {
            return res;
        }

        let mut odd_idx = 0;
        let mut even_idx = 0;

        while odd_idx < a.len() && a[odd_idx] % 2 == 0 {
            odd_idx += 1;
        }
        while even_idx < a.len() && a[even_idx] % 2 != 0 {
            even_idx += 1;
        }

        while odd_idx < a.len() && even_idx < a.len() {
            res.push(a[even_idx]);
            even_idx += 1;
            res.push(a[odd_idx]);
            odd_idx += 1;
            // find next even and odd
            while odd_idx < a.len() {
                if a[odd_idx] % 2 == 0 {
                    odd_idx += 1;
                } else {
                    break;
                }
            }
            while even_idx < a.len() {
                if a[even_idx] % 2 != 0 {
                    even_idx += 1;
                } else {
                    break;
                }
            }
        }

        return res;
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5,6,7,8];
    println!("{:?}", Solution::sort_array_by_parity_ii(v));
}
