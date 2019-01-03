struct Solution;

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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut r_b = root.as_ref().unwrap().borrow_mut();
        let (mut l, mut r ) = (r_b.left.take(), r_b.right.take());
        return Solution::count_nodes(l) + Solution::count_nodes(r) + 1;
    }
}

fn main() {
    println!("Hello, world!");
}
