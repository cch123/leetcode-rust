struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if typed.len() < name.len() || name.chars().nth(0).unwrap() != typed.chars().nth(0).unwrap()
        {
            return false;
        }

        let mut idx = 0;
        for i in 0..name.len() {
            if idx >= typed.len() {
                return false;
            }

            let c1 = name.chars().nth(i).unwrap();
            let c2 = typed.chars().nth(idx).unwrap();

            if c1 == c2 {
                idx += 1;
                continue;
            }

            while typed.chars().nth(idx).unwrap() == c2 {
                idx += 1;
                if idx >= typed.len() {
                    return false;
                }
            }
            if typed.chars().nth(idx).unwrap()!= c1 {
                return false;
            }
            idx += 1;
        }
        return true;
    }
}

fn main() {
    println!("{}", Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()));
    println!("{}", Solution::is_long_pressed_name("leelee".to_string(), "lleeelee".to_string()));
    println!("{}", Solution::is_long_pressed_name("laiden".to_string(), "laiden".to_string()));
}
