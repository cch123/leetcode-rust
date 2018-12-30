struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1usize {
            return 1;
        }

        // O(1) 其实只要倒着遍历就行了，还是不写 O1 的了
        let mut char_counter = HashMap::new();

        let c_c = chars.clone();
        for c in c_c {
            let cnt = char_counter.get(&c).unwrap_or(&0);
            char_counter.insert(c, *cnt + 1);
        }

        let mut mark_cursor = 1usize;
        let mut iter_cursor = 0usize;
        let mut cur_char = chars[0].clone();
        while iter_cursor < chars.len() {
            //println!('chars {:?}', chars);
            let cnt = char_counter.get(&cur_char).unwrap();
            let cnt_str = (*cnt).to_string();
            //println!('cnt_str{:?}', cnt_str);
            for i in 0..cnt_str.len() {
                chars[mark_cursor] = cnt_str.chars().nth(i).unwrap();
                mark_cursor += 1;
            }
            if mark_cursor >= chars.len() {
                break;
            }
            iter_cursor = mark_cursor+1;
            // find next char
            while iter_cursor < chars.len() &&(chars[iter_cursor] == cur_char) {
                iter_cursor += 1;
            }
            //println!('cur_char:{}, iter_cursor:{}, mark_cursor:{}', cur_char, iter_cursor,mark_cursor);
            if iter_cursor == chars.len() {
                break;
            }
            chars[mark_cursor] = chars[iter_cursor];
            cur_char = chars[mark_cursor];
            mark_cursor += 1;
            //println!('cur_char:{}, mark_cursor:{}', cur_char, mark_cursor);
        }
        //println!('{:?}', chars);
        return mark_cursor as i32;
    }
}

fn main() {
    let mut v = vec!['a','a','b','b','c','c','c'];
    println!("{}", Solution::compress(&mut v));
    let mut v = vec!['a', 'a'];
    println!("{}", Solution::compress(&mut v));
    let mut v = vec!['a'];
    println!("{}", Solution::compress(&mut v));
    let mut v = vec!['?','?','?','0','0','^','^','g','g','?','`','v','^','^','$','$','4','4','z','c',';',';',';',';',';','@','@','@','@','K','l','l','~','~','~','[','[','t','t','?','V','I','<','i','i','i','i','i','o','o','o','o','7','8','o',':','z','t','t','t','K','w','d','$','$','$','$','$','A','5','5','2','m','m','m',',','Y','O','O','O','O','O','O','O','M','k','k','Y','@','@','@','_','_','c','c','c','E','`','`','e','e','e','F'];
    println!("{}", Solution::compress(&mut v));
}
