fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn count_triplets(a: Vec<i32>) -> i32 {
        let mut res = 0;
        (0..a.len()).for_each(|i| {
            (i..a.len()).for_each(|j| {
                let tmp = a[i] & a[j];
                (j..a.len()).for_each(|k| {
                    if tmp & a[k] == 0 {
                        if i == j && j == k {
                            res += 1;
                        } else if i == j || j == k || i == k {
                            res += 3;
                        } else {
                            res += 6;
                        }
                    }
                })
            })
        });
        res
    }
}
