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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut current = head.as_mut();
        while p.is_some() || q.is_some() {
            let mut sum  = carry;
            if let Some(v) = p {
                sum += v.val;
                p = v.next;
            }
            if let Some(v) = q {
                sum += v.val;
                q = v.next;
            }

            carry = sum / 10;
            if let Some(cur) = current {
                cur.next = Some(Box::new(ListNode::new(sum % 10)));
                current = cur.next.as_mut();
            }
        }

        if carry > 0 {
            current.unwrap().next = Some(Box::new(ListNode::new(1)));
        }
        head.unwrap().next
    }
}
struct Solution {}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(1)));
    let mut l2 = Some(Box::new(ListNode::new(9)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    //let l3 = Solution::add_two_numbers(l1,l2);

    //println!("{:?}", l3);
    println!("l1 {:?}", l1);
    println!("l2 {:?}", l2);
    let mut x = Solution::add_two_numbers(l1, l2);
    println!("res {:?}", x);
}
