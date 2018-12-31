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

        let (mut l1, mut l2) = (&mut l1, &mut l2);

        loop {
            match (l1.as_ref(), l2.as_ref()) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        std::mem::swap(&mut cursor.next, l1);
                        std::mem::swap(&mut cursor.next.as_mut().unwrap().next, l1);
                        /* 其实把向后移动放这里可能更直观一些
                        std::mem::swap(&mut l3.next, &mut l1);
                        l3 = l3.next.as_mut().unwrap();
                        std::mem::swap(&mut l3.next, &mut l1);
                        这么写的话，if 外面那个 cur = cur.next 就不用写了
                        */
                    } else {
                        std::mem::swap(&mut cursor.next, l2);
                        std::mem::swap(&mut cursor.next.as_mut().unwrap().next, l2);
                    }
                    cursor = cursor.next.as_mut().unwrap();
                }
                (None, Some(n2)) => {
                    std::mem::swap(&mut cursor.next, l2);
                    break;
                }
                (Some(n1), None) => {
                    std::mem::swap(&mut cursor.next, l1);
                    break;
                }
                (None, None) => break,
            }
        }

        return h.next;
    }
}

fn main() {
    let l1 = linkedlist![1, 2, 3, 4];
    let l2 = linkedlist![1, 2, 3, 4];
    let l3 = Solution::merge_two_lists(l1, l2);
    println!("{:?}", l3);
}

/*
leetcode 上的递归解法
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(l1) = l1 {
            if let Some(l2) = l2 {
                if l1.val < l2.val {
                    let l1 = *l1;
                    let merged_rest = Self::merge_two_lists(l1.next, Some(l2));
                    Some(Box::new(ListNode{val:l1.val, next: merged_rest}))
                } else {
                    let l2 = *l2;
                    let merged_rest = Self::merge_two_lists(Some(l1), l2.next);
                    Some(Box::new(ListNode{val:l2.val, next: merged_rest}))
                }
            } else {
                Some(l1)
            }
        } else {
            l2
        }
    }
}
*/
