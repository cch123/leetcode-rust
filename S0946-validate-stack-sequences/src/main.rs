struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack= vec![];
        let mut cursor = 0;
        let mut cursor_pushed = 0;
        while cursor_pushed < pushed.len() {
            //println!("stack: {:?}, popped:{:?}, cursor:{}", stack,popped, cursor);
            if stack.len() == 0 {
                stack.push(pushed[cursor_pushed]);
                cursor_pushed+=1;
                continue;
            }
            if stack[stack.len()-1] != popped[cursor] {
                stack.push(pushed[cursor_pushed]);
                cursor_pushed+=1;
                continue;
            }
            // stack len > 0 and top = popped[cursor]
            stack.pop();
            cursor+=1;
        }

        //println!("stack: {:?}, popped:{:?}, cursor:{}", stack,popped, cursor);
        for i in cursor..popped.len() {
            if stack.pop().unwrap() != popped[i] {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    println!("{}", Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]));
}
