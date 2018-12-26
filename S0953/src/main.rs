use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map : HashMap<u8, i32> = HashMap::new();
        let order = order.as_bytes();
        for i in 0..order.len() {
            map.insert(order[i], i as i32);
        }

        for i in 1..(words.len()) {
            if !Solution::bigger_than(&words[i], &words[i-1], &map) {
                return false;
            }
        }
        return true;
    }

    fn bigger_than(a: &String, b: &String, dict: &HashMap<u8, i32>) -> bool {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let mut l = a.len();
        if b.len() < a.len() {
            l = b.len();
        }

        for idx in 0..l  {
            if dict.get(&a[idx]) > dict.get(&b[idx]) {
                return true;
            }
            if dict.get(&a[idx]) < dict.get(&b[idx]) {
                return false;
            }
            // == continue
        }

        if a.len() > b.len() {
            return true;
        }
        return false;
    }
}

fn main() {
    assert_eq!(true, Solution::is_alien_sorted(vec!["hello".to_string(),"leetcode".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()));
    assert_eq!(false, Solution::is_alien_sorted(vec!["apple".to_string(),"app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()));
    assert_eq!(false, Solution::is_alien_sorted(
        vec!["word".to_string(),"world".to_string(),"row".to_string()],
        "worldabcefghijkmnpqstuvxyz".to_string())
    );
}
