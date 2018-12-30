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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
        let mut cur_level = vec![root];
        while cur_level.len() > 0 {
            let mut next_level = vec![];
            let mut current_level_avg = 0f64;
            let mut current_level_sum = 0f64;
            for n in &cur_level {
                let mut n_b = n.as_ref().unwrap().borrow_mut();
                current_level_sum += n_b.val as f64;
                let (mut l, mut r) = (n_b.left.take(), n_b.right.take());
                if l.is_some() {
                    next_level.push(l);
                }
                if r.is_some() {
                    next_level.push(r);
                }
            }
            res.push(current_level_sum/(cur_level.len() as f64));
            cur_level = next_level;
        }
        return res;
    }
}
fn main() {
    let r = btree![3,9,20,null,null,15,7];
    println!("{:?}", Solution::average_of_levels(r));
}
