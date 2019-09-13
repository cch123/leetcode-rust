fn main() {
    println!("Hello, world!");
}

/*
 * @lc app=leetcode.cn id=1019 lang=rust
 *
 * [1019] 链表中的下一个更大节点
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
    fn count_len(head: Option<Box<ListNode>>) -> (i32, Vec<i32>) {
        let mut head = head;
        let mut cursor = &mut head;
        let mut cnt = 0;
        let mut v = vec![];
        loop {
            match cursor {
                Some(e) => {
                    cursor = &mut e.next;
                    cnt += 1;
                    v.push(e.val);
                }
                None => break,
            }
        }
        (cnt, v)
    }

    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack = vec![];
        //let mut index_arr = vec![];
        let (cnt, v) = Self::count_len(head);
        let mut res = vec![];
        (0..cnt).for_each(|_| res.push(0));
        for (idx, v) in v.iter().enumerate() {
            if stack.is_empty() {
                stack.push((v, idx));
                continue;
            }

            while !stack.is_empty() && stack.last().unwrap().0 < v {
                let (_, i) = stack.pop().unwrap();
                res[i] = *v;
            }
            stack.push((v, idx));
        }
        res
    }
}


