struct Solution;
use leetcode_prelude::*;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
#[derive(Debug)]
struct HeapNode {
    val: i32,
    idx: usize,
}

impl HeapNode {
    pub fn new(val: i32, idx: usize) -> Self {
        return HeapNode { val, idx };
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        return self.val == other.val;
    }
}
impl Eq for HeapNode {}

impl std::cmp::Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.val.cmp(&other.val);
    }
}

impl std::cmp::PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &HeapNode) -> Option<Ordering> {
        if self.val < other.val {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

impl Solution {
    // 基本思路，构建小顶堆
    // 每次取堆顶，并把对应位置的数组索引后移
    // 最大时间复杂度，应该是 lg(n) * (n*m)
    // 如果是 merge_k_lists2 的解法的话，时间复杂度是 (n*m)*log(n*m)
    // 但是从这道题给的数据上，是看不出这两种算法的区别的
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        // build heap
        let mut list_cursors = vec![];
        for (idx, l) in lists.iter().enumerate() {
            if l.is_some() {
                heap.push(HeapNode::new(l.as_ref().unwrap().val, idx));
            }
            list_cursors.push(l);
        }
        // println!("{:?}\n {:?}", heap, list_cursors);
        let mut dummy = Box::new(ListNode::new(0));
        let mut cursor = &mut dummy;
        loop {
            let x = heap.pop();
            if x.is_none() {
                break;
            }

            // 结果链表
            cursor.next = Some(Box::new(ListNode::new(x.as_ref().unwrap().val)));
            cursor = cursor.next.as_mut().unwrap();

            let (idx, prev_v) = (x.as_ref().unwrap().idx, x.as_ref().unwrap().val);

            let mut next = &list_cursors[idx].as_ref().unwrap().next;
            if next.is_some() {
                let v = next.as_ref().unwrap().val;
                heap.push(HeapNode::new(v, idx));
            }
            list_cursors[idx] = next;
        }
        dummy.next
    }

    pub fn merge_k_lists2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut num_vec = vec![];
        for l in lists {
            let mut inner_list = l;
            while inner_list.is_some() {
                num_vec.push(inner_list.as_ref().unwrap().val);
                inner_list = inner_list.unwrap().next;
            }
        }
        num_vec.sort();
        let mut dummy = Box::new(ListNode::new(0));
        let mut cursor = &mut dummy;
        for num in num_vec {
            let node = Some(Box::new(ListNode::new(num)));
            cursor.next = node;
            cursor = cursor.next.as_mut().unwrap();
        }
        return dummy.next;
    }
}

fn main() {
    let v = vec![linkedlist![1, 2, 3], linkedlist![2, 3, 4]];
    let l = Solution::merge_k_lists(v);
    println!("{:?}", l);
}
