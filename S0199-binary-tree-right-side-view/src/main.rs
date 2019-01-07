struct Solution;
use leetcode_prelude::*;

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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut current_level = vec![root];
        let mut res = vec![];
        loop {
            let last_node = &current_level[current_level.len() - 1];
            res.push(last_node.as_ref().unwrap().borrow_mut().val);

            let mut next_level = vec![];
            for n in current_level {
                let mut n_r_b = n.as_ref().unwrap().borrow_mut();
                if n_r_b.left.is_some() {
                    next_level.push(n_r_b.left.take());
                }
                if n_r_b.right.is_some() {
                    next_level.push(n_r_b.right.take());
                }
            }
            if next_level.len() == 0 {
                break;
            }
            current_level = next_level;
        }
        res
    }
}
fn main() {
    let mut b = btree![1, 2, 3, null, 5, null, 4];
    println!("{:?}", Solution::right_side_view(b));
}
