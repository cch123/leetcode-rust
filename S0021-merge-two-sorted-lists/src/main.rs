struct Solution;
use leetcode_prelude::*;
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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut h = Box::new(ListNode::new(0));
        let mut cursor = &mut h;

        let (mut l1, mut l2) = (&mut l1,&mut l2);

        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                std::mem::swap(&mut cursor.next,l1);
                std::mem::swap(&mut cursor.next.as_mut().unwrap().next,l1);
                //println!("1cursor :{:?}", cursor);
            } else {
                std::mem::swap(&mut cursor.next,l2);
                std::mem::swap(&mut cursor.next.as_mut().unwrap().next,l2);
                //println!("2cursor :{:?}", cursor);
            }
            cursor = cursor.next.as_mut().unwrap();
        }

        if l1.is_some() {
            std::mem::swap(&mut cursor.next, l1);
        } else {
            std::mem::swap(&mut cursor.next, l2);
        }

        return h.next;
    }
}

fn main() {
    let l1 = linkedlist![1,2,3,4];
    let l2 = linkedlist![1,2,3,4];
    let l3 = Solution::merge_two_lists(l1,l2);
    println!("{:?}", l3);
}
