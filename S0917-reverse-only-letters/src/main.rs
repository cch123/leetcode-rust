struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut char_arr = vec![];
        let mut nonchar_arr = vec![];
        s.bytes().for_each(|c| match c {
            b'a'..=b'z' | b'A'..=b'Z' => char_arr.push(c),
            _ => nonchar_arr.push(c),
        });

        let mut char_iter = char_arr.iter().rev();
        let mut nonchar_iter = nonchar_arr.iter();
        let mut byte_arr = vec![];
        s.chars().for_each(|c| match c {
            'a'..='z' | 'A'..='Z' => byte_arr.push(char_iter.next().unwrap().clone()),
            _ => byte_arr.push(nonchar_iter.next().unwrap().clone()),
        });

        String::from_utf8(byte_arr).unwrap()
    }
}

fn main() {
    println!("{}", Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()))
}

/*

impl Solution {
pub fn reverse_only_letters(s: String) -> String {
    if s.len()==0{
        return "".to_string();
    }
    let mut s = s.as_bytes().to_vec();
    let mut result = String::new();

    let mut i = 0;
    let mut j = s.len() -1;

    while i<j{
        while i<j && !(s[i] as char).is_alphabetic(){
            i += 1;
        }
        while i<j && !(s[j] as char).is_alphabetic(){
            j -= 1;
        }
        if i<j{
            s.swap(i,j);
            i += 1;
            j -= 1;
        }
    }
    result = String::from_utf8(s).unwrap();
    result
}
}*/