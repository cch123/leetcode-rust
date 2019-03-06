fn main() {
    let res = Solution::uncommon_from_sentences("abc def".to_string(), "def aaa".to_string());
    println!("{:?}", res);
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let line = a + " " + b.as_str();
        let word_arr = line.split(" ").collect::<Vec<&str>>();

        let mut counter = HashMap::new();
        word_arr.iter().for_each(|w| { counter.entry(w).and_modify(|e| *e += 1).or_insert(1); });
        counter.iter().filter(|(_, &c)| c == 1).map(|(w, _)| w.to_string()) .collect::<Vec<String>>()
    }
}
