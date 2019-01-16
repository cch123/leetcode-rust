struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut char_set = HashSet::new();
        let mut cur_len = 0;
        let mut cc = s.chars().collect::<Vec<char>>();
        for (idx, c) in cc.iter().enumerate() {
            if char_set.contains(&c) {
                for i in (idx-cur_len)..idx {
                    // 这里如果用 s.chars().nth() 会非常非常非常非常慢
                    let char_at_i = cc[i];
                    char_set.remove(&char_at_i);
                    cur_len -= 1;
                    if char_at_i == *c {
                        break
                    }
                }
            }
            char_set.insert(c);
            cur_len += 1;
            max_len = max_len.max(cur_len);
        }

        return max_len as i32
    }
}

fn main() {
    println!("{}", Solution::length_of_longest_substring("abcabcbb".to_string()));
    println!("{}", Solution::length_of_longest_substring("bbbbb".to_string()));
    println!("{}", Solution::length_of_longest_substring("pwwkew".to_string()));
}

/*

impl Solution {
    pub fn length_of_longest_substring(str: String) -> i32 {
        let s = str.as_bytes();
        let (mut start, mut end) = (0, 0);
        let (mut current, mut max ) = (0, 0);

        for k in 0..s.len() {
            for i in start..end  {
                if s[end] == s[i] {
                    current = end - start;
                    if current > max {
                        max = current;
                    }
                    start = i + 1;
                    break;
                }
            }
            end += 1;
        }
        std::cmp::max(end - start, max) as i32
    }
}
*/

/*

impl Solution {
    #[inline]
    fn find_and_romove(v: &mut Vec<char>, target: char) -> Option<(usize, Vec<char>)> {
        for (i, ch) in v.iter().enumerate()  {
            if target == *ch {
                return Some((v.len(), v.split_off(i+1)))
            }
        }
        None
    }


    fn length_of_longest_substring(s: String) -> i32 {
        let mut v = vec![];
        let (mut current, mut max ) = (0, 0);
        for ch in s.chars() {
            if let Some((c, list)) = Solution::find_and_romove(&mut v, ch) {
                current = c;
                v = list;
                if current > max {
                    max = current;
                }
            }
            v.push(ch);
        }
        std::cmp::max(v.len(), max) as i32
    }
}
*/

/*
se std::cmp;
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        let mut output = 0;
        let mut i = 0;
        let mut j = 0;

        while i<len && j<len {
            if set.contains(&chars[j]) {
                set.remove(&chars[i]);
                i += 1;
            } else {
                set.insert(chars[j]);
                j += 1;
                output = cmp::max(output, j - i);
            }
        }

        return output as i32;
    }
}
*/