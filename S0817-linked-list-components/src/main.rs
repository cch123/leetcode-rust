struct Solution;
// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }
use std::collections::HashSet;
impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, g: Vec<i32>) -> i32 {
        let mut cur = head;
        let mut passing_component = false;
        let set = g.iter().collect::<HashSet<_>>();
        let mut res = 0;
        while cur.is_some() {
            if passing_component == false {
                if set.contains(&cur.as_ref().unwrap().val) {
                    passing_component = true;
                }
            } else {
                if !set.contains(&cur.as_ref().unwrap().val) {
                    passing_component = false;
                    res += 1;
                }
            }
            cur = cur.unwrap().next;
        }

        if passing_component == true {
            return res + 1;
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
