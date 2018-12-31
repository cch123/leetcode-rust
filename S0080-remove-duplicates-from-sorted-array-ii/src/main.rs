struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let (mut cursor, mut i) = (0usize, 0usize);
        let mut counter = 0;
        let mut cur_val = nums[0];
        while i < nums.len() {
            if nums[i] == cur_val {
                counter += 1;
            } else {
                counter = 1;
                cur_val = nums[i];
            }

            match counter {
                0 => println!("unreqchable"),
                1 => {
                    nums.swap(cursor,i);
                    i+=1;cursor+=1;
                },
                2 => {
                    nums.swap(cursor,i);
                    i+=1;cursor+=1;
                },
                _ => {
                    // find next
                    while i < nums.len() && nums[i] == cur_val {
                        i+=1;
                    }
                },
            }

        }
        return cursor as i32;
    }
}

fn main() {
    let mut v = vec![1,1,1,2,2,3];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
    let mut v = vec![1,1,1,1,2,2,2,3,3];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
    let mut v = vec![];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
    let mut v = vec![1];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
}

/*
这个解法比我的慢
use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut i = 0;
        while i != nums.len() {
            let val = nums[i];

            let v = m.get(&val).map_or(1, |x| x + 1);

            if v > 2 {
                nums.remove(i);
            } else {
                m.insert(val, v);
                i += 1;
            }
        }

        nums.len() as i32
    }
}
*/