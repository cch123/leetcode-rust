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
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.len() == 0 {
            return None;
        }

        // root is pre[0]
        let (mut left_tree, mut right_tree) = (None, None);

        if pre.len() > 1 {
            let left_tree_root_val = pre[1];
            let pos = post.iter().position(|x| *x == left_tree_root_val).unwrap();
            let (left_post, right_post) = post.split_at(pos + 1);
            let (left_post, right_post) = (
                left_post.to_vec(),
                right_post[..(right_post.len() - 1)].to_vec(),
            );
            let (left_pre, right_pre) = (
                pre[1..=left_post.len()].to_vec(),
                pre[(left_post.len() + 1)..].to_vec(),
            );
            left_tree = Solution::construct_from_pre_post(left_pre, left_post);
            right_tree = Solution::construct_from_pre_post(right_pre, right_post);
        }

        // need to split the left and the right
        Some(Rc::new(RefCell::new(TreeNode {
            val: pre[0],
            left: left_tree,
            right: right_tree,
        })))
    }
}



fn main() {
    println!("{:?}", Solution::construct_from_pre_post(vec![1,2,4,5,3,6,7], vec![4,5,2,6,7,3,1]));
}
