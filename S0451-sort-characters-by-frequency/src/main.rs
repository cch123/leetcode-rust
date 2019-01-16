struct Solution;

use std::collections::HashMap;
use std::cmp::Ordering;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut char_cnt = HashMap::new();
        s.bytes().for_each(|c|{
            char_cnt.entry(c).and_modify(|e|*e+=1).or_insert(1);
        });

        let mut s = s;
        unsafe {
            let mut z = s.as_bytes_mut();
            z.sort_by(|x, y| {
                if char_cnt.get(x).unwrap_or(&0) > char_cnt.get(y).unwrap_or(&0) {
                    return Ordering::Less;
                }
                if char_cnt.get(x).unwrap_or(&0) < char_cnt.get(y).unwrap_or(&0) {
                    return Ordering::Greater;
                }
                return x.cmp(y)
            });
            return String::from_utf8(z.to_vec()).unwrap();
        }

        "".to_string()
    }
}

fn main() {
    println!("{}", Solution::frequency_sort("abacd".to_string()));
}
