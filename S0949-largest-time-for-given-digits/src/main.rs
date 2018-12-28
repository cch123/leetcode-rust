struct Solution;
impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut res = "".to_string();
        for i in 0..a.len() {
            for j in 0..a.len() {
                for k in 0..a.len() {
                    for l in 0..a.len() {
                        if i == j || i == k || i == l || j == k || j == l || k == l {
                            continue;
                        }
                        let mut tmp = (a[i]).to_string();
                        tmp.push_str(&(a[j]).to_string());
                        tmp.push(':');
                        tmp.push_str(&(a[k]).to_string());
                        tmp.push_str(&(a[l]).to_string());
                        if a[i] > 2 {
                            continue;
                        }
                        if a[i] == 2 {
                            if a[j] >= 4 {
                                continue;
                            }
                        }
                        if a[k] >=6 {
                            continue;
                        }
                        if tmp > res {
                            res = tmp;
                        }
                    }
                }
            }
        }
        return res;
    }
}

fn main() {
    println!("{}", Solution::largest_time_from_digits(vec![1,2,3,4]));
    println!("{}", Solution::largest_time_from_digits(vec![5,5,5,5]));
}
