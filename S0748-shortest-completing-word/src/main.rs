struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut m = HashMap::new();
        license_plate
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' => {
                    let c = c.to_ascii_lowercase();
                    m.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
                _ => {}
            })
            .for_each(drop);

        let mut res = "".to_string();
        'outer: for x in words {
            let mut char_cnt = HashMap::new();
            x.chars()
                .map(|c| {
                    char_cnt.entry(c).and_modify(|e| *e += 1).or_insert(1);
                })
                .for_each(drop);

            for (x, y) in m.iter() {
                if char_cnt.get(x).unwrap_or(&0) < y {
                    continue 'outer;
                }
            }

            if res.is_empty() || x.len() < res.len() {
                res = x.to_string();
            }
        }

        return res.to_string();
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::shortest_completing_word(
            "1s3 PSt".to_string(),
            vec![
                "step".to_string(),
                "steps".to_string(),
                "stripe".to_string(),
                "stepple".to_string()
            ]
        )
    );
}
