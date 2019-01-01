struct Solution;
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
}
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        // 遍历链表
        // 一个数组记小于的，另一个记大于等于的
        let (mut v_left, mut v_right) = (vec![], vec![]);
        let mut cur = head.unwrap();
        loop {
            if cur.val < x {
                v_left.push(cur.val);
            } else {
                v_right.push(cur.val);
            }

            if cur.next.is_some() {
                cur = cur.next.unwrap();
            } else {
                break;
            }
        }
        println!("{:?}, {:?}", v_left, v_right);
        let mut dummy = Box::new(ListNode::new(0));
        let mut cursor = &mut dummy;
        for num in v_left {
            cursor.next = Some(Box::new(ListNode::new(num)));
            cursor = cursor.next.as_mut().unwrap();
        }
        for num in v_right {
            cursor.next = Some(Box::new(ListNode::new(num)));
            cursor = cursor.next.as_mut().unwrap();
        }
        return dummy.next;
    }
}

fn main() {
    println!("Hello, world!");
}
