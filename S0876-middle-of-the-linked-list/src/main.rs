struct Solution;
// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }
impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 先遍历一遍计数
        let mut cnt = 0;
        let mut cur = &mut head;
        loop {
            if cur.is_none() {
                break;
            }
            cnt+=1;
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur = &mut head;
        let mut i = 0;
        while i != cnt/2 {
            cur = &mut cur.as_mut().unwrap().next;
            i+=1;
        }
        return cur.take();
    }
}

fn main() {
    println!("Hello, world!");
}
