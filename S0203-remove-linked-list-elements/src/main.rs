fn main() {
    println!("Hello, world!");
}

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

struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        while head.is_some() && head.as_ref().unwrap().val == val {
            head = head.unwrap().next;
        }
        if head.is_none() {
            return head
        }

        let mut cur = &mut head;
        while cur.as_ref().unwrap().next.is_some() {
            if cur.as_ref().unwrap().next.as_ref().unwrap().val == val {
                let next= std::mem::replace(&mut cur.as_mut().unwrap().next, None);
                std::mem::replace(&mut cur.as_mut().unwrap().next, next.unwrap().next);
                continue;
            }
            cur = &mut cur.as_mut().unwrap().next;
        }

        return head;
    }
}

/*
use std::mem;
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while cur.is_some() {
            match cur.as_ref() {
                Some(node) if node.val == val => {
                    let mut move_out = cur.take();
                    mem::swap(cur, &mut move_out.as_mut().unwrap().next);
                    continue;
                }
                _ => {}
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}
*/

/*
这个很优雅
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode{val:0, next:head};
        let mut p = &mut dummy;
        while let Some(q) = p.next.as_mut() {
            if q.val == val {
                p.next = q.next.take();
            }else{
                p = p.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
*/