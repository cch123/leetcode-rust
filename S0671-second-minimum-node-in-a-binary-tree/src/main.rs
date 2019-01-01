// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }
struct Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_second_minimum_value(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = vec![];
        Solution::traverse_and_push_val_to_vec(&mut root, &mut v);
        v.sort();
        if v.len() == 1 {
            return -1;
        }
        let (first, last) = (v[0],v[v.len()-1]);
        if first == last {
            return -1;
        }
        for n in &v {
            if *n != v[0] {
                return *n;
            }
        }
        return -1;
    }

    fn traverse_and_push_val_to_vec(n:&mut Option<Rc<RefCell<TreeNode>>>, vec:&mut Vec<i32>) {
        if n.is_none() {
            return
        }
        let mut n_b= n.as_ref().unwrap().borrow_mut();
        vec.push(n_b.val);
        Solution::traverse_and_push_val_to_vec(&mut n_b.left,vec);
        Solution::traverse_and_push_val_to_vec(&mut n_b.right, vec);
    }
}
fn main() {
    println!("Hello, world!");
}
