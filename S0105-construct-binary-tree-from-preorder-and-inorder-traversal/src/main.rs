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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let head_val = preorder[0];

        let pos = inorder.iter().position(|&v| v == head_val).unwrap();
        let (left, right) = inorder.split_at(pos);
        let (left, right) = (left.to_vec(), right[1..].to_vec());

        // preorder also need to split
        let (left_pre, right_pre) = (
            preorder[1..=left.len()].to_vec(),
            preorder[(left.len() + 1)..].to_vec(),
        );

        let left_tree = Solution::build_tree(left_pre, left);
        let right_tree = Solution::build_tree(right_pre, right);

        Some(Rc::new(RefCell::new(TreeNode {
            val: head_val,
            left: left_tree,
            right: right_tree,
        })))
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}
