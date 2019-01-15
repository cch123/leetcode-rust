struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut m = HashMap::new();
        license_plate
            .chars()
            .map(|c| match c {
                'a'..='z' => {
                    let cnt = m.get(&c).unwrap_or(&0);
                    m.insert(c, cnt + 1);
                }
                'A'..='Z' => {
                    let c = c.to_ascii_lowercase();
                    let cnt = m.get(&c).unwrap_or(&0);
                    m.insert(c, cnt + 1);
                }
                _ => {}
            })
            .for_each(drop);

        let mut res = "".to_string();
        for x in words {
            let mut char_cnt = HashMap::new();
            x.chars()
                .map(|c| {
                    let cnt = char_cnt.get(&c).unwrap_or(&0);
                    char_cnt.insert(c, cnt + 1);
                })
                .for_each(drop);

            let mut valid = true;
            for (x, y) in m.iter() {
                if char_cnt.get(x).unwrap_or(&0) < y {
                    valid = false;
                    break;
                }
            }

            if valid {
                if res == "".to_string() || x.len() < res.len() {
                    res = x.to_string();
                }
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
