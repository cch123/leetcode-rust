// Definition for singly-linked list.
use leetcode_prelude::*;

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cursor = &mut head;
        while cursor.is_some() && cursor.as_ref().unwrap().next.is_some() {
            let mut next = None;
            // cursor.next = None; tmp = cursor.next
            std::mem::swap(&mut cursor.as_mut().unwrap().next, &mut next);
            // cursor.next = tmp.next; tmp.next = None;
            std::mem::swap(
                &mut cursor.as_mut().unwrap().next,
                &mut next.as_mut().unwrap().next,
            );
            // tmp.next = cursor
            std::mem::swap(cursor, &mut next.as_mut().unwrap().next);
            //cursor = &mut next;
            std::mem::swap(cursor, &mut next);
            cursor = &mut cursor.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        //println!("head : {:?}", &head);
        head
    }
}

fn main() {
    let l = linkedlist![1, 2, 3, 4];
    println!("{:?}", Solution::swap_pairs(l));
}

/*
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::mem::swap;

        if head.is_none() {
            return None;
        }

        let mut head = Solution::swap_head(head.unwrap());
        let mut current = &mut head.next;

        let mut should_swap = true;
        while let Some(ref mut node) = current {
            if should_swap {
                Solution::swap_nodes(node);
            }
            current = &mut node.next;
            should_swap = !should_swap;
        }

        Some(head)
    }

    // swaps the two nodes after parent.
    fn swap_nodes(parent: &mut Box<ListNode>) {
        use std::mem::swap;

        // parent -> a -> b -> c => parent -> b -> a -> c

        let mut a = None;
        let mut b = None;
        let mut c = None; // rest of the list

        swap(&mut parent.next, &mut a); // now we have: parent, a-> b -> c

        if let Some(ref mut a) = a {
            swap(&mut a.next, &mut b); // parent, a, b -> c
        }

        if let Some(ref mut b) = b {
            swap(&mut b.next, &mut c); // parent, a, b, c
        }
        else {
            // we reached the end of the list, put things back the way they were and return
            parent.next = a;
            return;
        }

        // now reassemble the list in the new order, parent -> b -> a -> c

        if let Some(ref mut a) = a {
            a.next = c; // parent, b, a -> c
        }

        if let Some(ref mut b) = b {
            b.next = a; // parent, b -> a -> c
        }

        parent.next = b; // parent -> b -> a -> c
    }

    // swap the head of the list and the next element.
    // returns the new head
    fn swap_head(mut head: Box<ListNode>) -> Box<ListNode> {
        use std::mem::swap;

        if head.next.is_none() {
            head
        }
        else {
            let mut next = None;
            swap(&mut head.next, &mut next);
            let mut next = next.unwrap();
            let mut rest = next.next;
            head.next = rest;
            next.next = Some(head);
            next
        }
    }
}
*/