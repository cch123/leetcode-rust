use leetcode_prelude::*;
struct Solution;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        //println!("{:?}", root);
        if root.is_none() {
            return true;
        }
        let r_u = root.unwrap();
        let mut r_b = r_u.borrow_mut();

        if let Some(l) = r_b.left.take() {
            if l.borrow_mut().val != r_b.val {
                return false;
            }
            r_b.left = Some(l);
        }

        if let Some(r) = r_b.right.take() {
            if r.borrow_mut().val != r_b.val {
                return false;
            }
            r_b.right = Some(r);
        }
        return Solution::is_unival_tree(r_b.left.take()) && Solution::is_unival_tree(r_b.right.take());
    }
}

fn main() {
    //let r = btree![1,2,3,4,null];
    //println!("{}", Solution::is_unival_tree(r));
    //let r = btree![1,1,1,1,1];
    //println!("{}", Solution::is_unival_tree(r));
    let r = btree![2,2,2,5,2];
    println!("{}", Solution::is_unival_tree(r));
}
