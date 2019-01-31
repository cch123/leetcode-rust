use leetcode_prelude::*;
struct Solution;

use std::mem::replace;
// Definition for singly-linked list.
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() || k == 0 {
            return head;
        }
        let mut cursor = &mut head;
        // count len
        let mut len = 0;
        while cursor.as_ref().is_some() {
            len += 1;
            cursor = &mut cursor.as_mut().unwrap().next;
        }

        let (k, mut cnt) = (len-k % len, 0);
        cursor = &mut head;

        loop {
            if cnt == k {
                let mut target = replace(cursor, None);
                // find the tail
                let mut tail = &mut target;
                while tail.is_some() {
                    tail = &mut tail.as_mut().unwrap().next;
                }
                // now tail is None
                // replace it to head
                // set the tail.next = head
                replace(tail, head);
                return target;
            }
            cursor = &mut cursor.as_mut().unwrap().next;
            cnt += 1;
        }

    }
}

fn main() {
    let l = linkedlist![1, 2, 3, 4, 5];
    println!("{:?}", Solution::rotate_right(l, 2));
    let l = linkedlist![0,1,2];
    println!("{:?}", Solution::rotate_right(l, 4));
    let l = linkedlist![1,2];
    println!("{:?}", Solution::rotate_right(l, 1));
}

