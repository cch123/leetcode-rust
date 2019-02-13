use leetcode_prelude::*;
fn main() {
    let l = linkedlist![1,2,3,4];
    println!("{:?}", Solution::reverse_list(l));
}

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let cur = &mut head;
        let mut prev = None;
        while cur.is_some() {
            let mut cur_next = std::mem::replace(&mut cur.as_mut().unwrap().next, prev.take());
            prev = cur.take();
            std::mem::swap(&mut cur_next, cur);
        }

        prev
    }
}
struct Solution;

/*
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head; //move
        let mut new_head:Option<Box<ListNode>> = None;
        loop {
            if head.is_none() {
                break;
            }
            let val = head.as_mut().unwrap().val;
            let mut new_node = ListNode::new(val);
            if new_head.is_some() {
                new_node.next = new_head;
            }
            new_head = Some(Box::new(new_node));
            head = head.unwrap().next;
        }
        new_head
    }
}
*/