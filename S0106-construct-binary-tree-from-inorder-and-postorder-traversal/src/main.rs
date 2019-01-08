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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() == 0 {
            return None;
        }

        let head_val = postorder[postorder.len() - 1];

        let pos = inorder.iter().position(|&x| x == head_val).unwrap();
        let (left, right) = inorder.split_at(pos);
        let (left, right) = (left.to_vec(), right[1..].to_vec());

        let (left_post, right_post) = (
            postorder[0..left.len()].to_vec(),
            postorder[left.len()..postorder.len() - 1].to_vec(),
        );
        //println!("{:?},{:?},{:?},{:?},{:?},{:?}", inorder,postorder,left,left_post, right,right_post);

        let (left_tree, right_tree) = (
            Solution::build_tree(left, left_post),
            Solution::build_tree(right, right_post),
        );
        //println!("left:{:?}, right:{:?}", left_tree, right_tree);

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
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}
