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

/*
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        use std::mem::swap;
        
        let len = Solution::len(&head);
        if k == 0 || len == 0 || k as usize % len == 0 {
            return head;
        }
        
        let mut node = &mut head;
        for _ in 0..(len - (k as usize % len) - 1) {
            if let Some(ref mut n) = node {
                node = &mut n.next;
            } 
        }
        
        let mut new_head = None;
        if let Some(ref mut node) = node {
            swap(&mut node.next, &mut new_head);
        }
        
        // we have now cut the linked list in half
        // now we have to put it back together backwards
        
        if let Some(ref mut new_head) = new_head {
            let mut node = new_head;
            while let Some(ref mut next) = node.next {
                node = next;
            }
            node.next = head;
        }
        
        new_head
    }
    
    fn len(head: &Option<Box<ListNode>>) -> usize {
        if let Some(ref n) = head {
            let mut len = 1;
            let mut node = n;
            while let Some(ref next) = node.next {
                len += 1;
                node = next;
            }
            len
        }
        else {
            0
        }
    }
}
*/
