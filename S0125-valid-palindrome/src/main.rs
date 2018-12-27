struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes().to_ascii_lowercase();
        let mut trimed: Vec<u8> = vec![];
        for i in 0..s.len() {
            if s[i] >= 'a' as u8 && s[i] <= 'z' as u8 {
                trimed.push(s[i]);
            }
            if s[i] >= '0' as u8 && s[i] <= '9' as u8 {
                trimed.push(s[i]);
            }
        }

        for i in 0..(trimed.len() / 2) {
            if trimed[i] != trimed[trimed.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
}

/*
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .bytes()
            .filter_map(|b| {
                if b.is_ascii_alphanumeric() {
                    Some(b.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }
}
*/
