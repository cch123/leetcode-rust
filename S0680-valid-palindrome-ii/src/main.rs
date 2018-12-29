struct Solution;

impl Solution {
    // 这个方案会超时
    pub fn valid_palindrome2(s: String) -> bool {
        for i in 0..s.len() {
            let mut s = s.clone();
            s.remove(i);

            // reverse string，翻转字符串
            let s_rev = s.chars().rev().collect::<String>();
            if s_rev == s {
                return true;
            }
        }
        return false;
    }

    pub fn valid_palindrome(s: String) -> bool {
        if s.len() == 1 {
            return true;
        }

        let s = s.as_bytes();
        let mut i = 0 as usize;
        let mut j = s.len() - 1 as usize;
        while i < j && s[i] == s[j] {
            i += 1;
            j -= 1;
        }
        //println!("i:{}, j:{}, s:{:?}", i, j, s);
        if i > j && s.len()%2== 1{
            return false;
        }
        if i == j {
            return true;
        }

        //println!("i:{}, j:{}, s:{:?}", i, j, s);
        // 遇到不等于的情况，跳出了 while 循环
        // 要么去掉 i 要么去掉 j
        // 先跳过 i 来一遍
        {
            let (mut i, mut j )= (i+1,j);
            while i< j{
                if s[i]!= s[j] {
                    break
                }
                i = i+1; j = j-1;
            }
            if i >= j {
                return true;
            }
        }
        // 再跳过 j 来一遍
        {
            let (mut i, mut j )= (i,j-1);
            while i< j{
                if s[i]!= s[j] {
                    break
                }
                i = i+1; j = j-1;
            }
            if i >= j {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    /*let mut s = "abcdefg".to_string();
    println!("{}, {}", s.remove(0),s);
    println!("{}, {}", s.remove(0),s);
    */
    println!("{}", Solution::valid_palindrome("aba".to_string()));
    println!("{}", Solution::valid_palindrome("abca".to_string()));
    println!("{}", Solution::valid_palindrome("bddb".to_string()));
    println!("{}", Solution::valid_palindrome("deeee".to_string()));
    println!("{}", Solution::valid_palindrome("cbbcc".to_string()));
}
