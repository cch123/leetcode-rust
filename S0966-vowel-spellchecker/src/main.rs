struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut word_map = HashMap::new();
        let mut word_lower_map = HashMap::new();
        let mut replace_vowel_map = HashMap::new();
        for w in wordlist.iter() {
            word_map.insert(w, w);
            word_lower_map.insert(w.to_ascii_lowercase(), w);

            let s: String = w
                .chars()
                .map(|x| match x {
                    'a' | 'e' | 'i' | 'o' | 'u' => '?',
                    _ => x,
                })
                .collect::<String>();

            replace_vowel_map.insert(s, w);
        }

        let res = queries
            .into_iter()
            .map(|w| {
                if word_map.contains_key(&w) {
                    return word_map.get(&w).unwrap().to_string();
                }
                let x = w.to_ascii_lowercase();
                if word_lower_map.contains_key(&x) {
                    return word_lower_map.get(&x).unwrap().to_string();
                }
                let s: String = w
                    .chars()
                    .map(|x| match x {
                        'a' | 'e' | 'i' | 'o' | 'u' => '?',
                        _ => x,
                    })
                    .collect::<String>();
                if replace_vowel_map.contains_key(&s) {
                    return replace_vowel_map.get(&s).unwrap().to_string();
                }
                return "".to_string();
            })
            .collect::<Vec<String>>();
        return res;
    }
}

fn main() {
    let mut a = vec!["KiTe", "kite", "hare", "Hare"];
    let mut a = a
        .into_iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
    let b = vec![
        "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
    ];
    let mut b = b
        .into_iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", Solution::spellchecker(a, b));

    let mut a = vec!["v", "t", "k", "g", "n", "k", "u", "h", "m", "p"];
    let mut a = a
        .into_iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
    let b = vec!["n", "g", "k", "q", "m", "h", "x", "t", "p", "p"];
    let mut b = b
        .into_iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", Solution::spellchecker(a, b));
}
