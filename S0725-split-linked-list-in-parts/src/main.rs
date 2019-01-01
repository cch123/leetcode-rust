struct Solution;
use leetcode_prelude::*;
// Definition for singly-linked list.
//#[derive(PartialEq, Eq, Debug)]
//pub struct ListNode {
//    pub val: i32,
//    pub next: Option<Box<ListNode>>,
//}
//
//impl ListNode {
//    #[inline]
//    fn new(val: i32) -> Self {
//        ListNode { next: None, val }
//    }
//}
impl Solution {
    pub fn split_list_to_parts(
        mut root: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut res = vec![];
        if root.is_none() {
            for _ in 0..k {
                res.push(None);
            }
            return res;
        }

        // 需要知道链表到底有多长
        let mut len = 0;
        let mut cursor = &mut root;
        loop {
            if cursor.is_none() {
                break;
            }

            len += 1;
            cursor = &mut cursor.as_mut().unwrap().next;
        }

        let every_vec_len = len / k;
        // 有这么多需要 every_vec_len + 1 的，剩下的全是 every_vec_len 长度
        let except_vec_count = len - every_vec_len * k;
        cursor = &mut root;
        for _ in 0..except_vec_count {
            let mut v = vec![];
            for _ in 0..(every_vec_len + 1) {
                v.push(cursor.as_ref().unwrap().val);
                /*
                if let Some(c) = cursor {
                    cursor = &mut c.next;
                }
                */
                cursor = &mut cursor.as_mut().unwrap().next;
            }
            // build list from vec
            let l = Solution::build_list_from_vec(v);
            res.push(l);
        }
        loop {
            let mut v = vec![];
            for _ in 0..(every_vec_len) {
                v.push(cursor.as_ref().unwrap().val);
                /*if let Some(c) = cursor {
                    cursor = &mut c.next;
                }*/
                cursor = &mut cursor.as_mut().unwrap().next;
            }
            // build list from vec
            let l = Solution::build_list_from_vec(v);
            res.push(l);
            if cursor.is_none() {
                break;
            }
        }
        while res.len() < k as usize {
            res.push(None);
        }
        return res;
    }

    fn build_list_from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cursor = &mut dummy;

        for num in v {
            cursor.next = Some(Box::new(ListNode::new(num)));
            cursor = cursor.next.as_mut().unwrap();
        }
        dummy.next
    }
}

fn main() {
    let l = linkedlist![1, 2, 3];
    let x = Solution::split_list_to_parts(l, 5);
    println!("{:?}", x);
}
