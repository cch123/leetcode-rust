use leetcode_prelude::*;
fn main() {
    let l = linkedlist![1,2,3,4,5];
    let x = Solution::reverse_between(l, 2,4);
    println!("{:?}", x);
}

// 本质上还是遍历了 1.x 遍
// 好像没有办法像其它语言那样记录多个指针？
struct Solution;
impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head
        }
        let mut cur  = &mut head;
        let mut cur_idx = 1;
        while cur_idx != m {
            cur_idx += 1;
            cur = &mut cur.as_mut().unwrap().next;
        }

        let mut prev = None;
        while cur_idx <= n {
            cur_idx += 1;
            // swap(prev, next)
            // prev == next
            std::mem::swap(&mut cur.as_mut().unwrap().next, &mut prev);
            // swap(prev, cur
            std::mem::swap(&mut prev, &mut cur);
        }

        let tail = std::mem::replace(cur, None);

        // connect mid and tail
        let mut mid_cur = &mut prev;
        while mid_cur.is_some() {
            mid_cur = &mut mid_cur.as_mut().unwrap().next;
        }
        std::mem::replace(mid_cur, tail);

        // connect head and mid
        let mut head_cur = &mut head;
        while head_cur.is_some() {
            head_cur = &mut head_cur.as_mut().unwrap().next;
        }
        std::mem::replace(head_cur, prev);

        head
    }
}