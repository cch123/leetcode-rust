//#![feature(box_patterns)]
//#![feature(nll)]
use std::collections::LinkedList;
use std::ops::Deref;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn next(&self) -> Option<&ListNode> {
        match &self.next {
            Some(n) => {
                // n 按说是一个 box，box n  的话更直接一些
                // 这里为啥 n.as_ref 也能拆出来？
                return Some(n.as_ref());
            }
            None => None,
        }
    }
    fn set_next(&mut self, node: ListNode) {
        self.next = Some(Box::new(node));
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v = vec![];
        //let head_val = l1.as_ref().unwrap().val + l2.as_ref().unwrap().val;
        //node_arr.push(ListNode::new(head_val));

        let mut l1_n = l1.unwrap();
        let mut l2_n = l2.unwrap();
        v.push(l1_n.val + l2_n.val);
        let l3 = Some(Box::new(ListNode::new(1)));

        loop {
            match (&l1_n.next, &l2_n.next) {
                (Some(n1), Some(n2)) => {
                    v.push(n1.val + n2.val);
                    l1_n = l1_n.next.unwrap();
                    l2_n = l2_n.next.unwrap();
                    //let tmp_v = n1.val + n2.val;
                    //node_arr.push(ListNode::new(tmp_v));
                }
                (Some(n1), None) => {
                    v.push(n1.val);
                    //node_arr.push(ListNode::new(n1.val));
                    l1_n = l1_n.next.unwrap();
                }
                (None, Some(n2)) => {
                    v.push(n2.val);
                    //node_arr.push(ListNode::new(n2.val));
                    l2_n = l2_n.next.unwrap();
                }
                (None, None) => break,
            }
        }
        println!("{:?}", v);

        /*
        if v.len() <= 1 {
            return l3;
        }

        struct L {
            head : Option<Box<ListNode>>,
            tail : Option<Box<ListNode>>,
        }
        impl L {
            pub fn push_back(&mut self, node : ListNode) {
                unsafe {
                    self.tail = Some(Box::new(node));
                }
            }
        }

        let mut l :L = L {
            head : Some(Box::new(ListNode::new(v[0]))),
            tail : Some(Box::new(ListNode::new(v[1]))),
        };
        l.head.as_mut().unwrap().next = l.tail;

        for i in 2..v.len() {
            l.push_back(ListNode::new(v[i]));
        }
        */

        return l3;
    }
}

struct Solution {}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(1)));
    let mut l2 = Some(Box::new(ListNode::new(10)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    //let l3 = Solution::add_two_numbers(l1,l2);

    //println!("{:?}", l3);
    let l4 = l1.as_ref().unwrap().next();
    println!("{:?}", l4);
    println!("{:?}", l1);
    let l5 = l4.unwrap().next();
    println!("{:?}", l5);
    let l5 = l1.as_ref().unwrap().next();
    println!("{:?}", l5);
    Solution::add_two_numbers(l1, l2);
}
