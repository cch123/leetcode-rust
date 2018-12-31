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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 中序遍历
        let mut v:Vec<i32> = vec![];
        Solution::mid_traversal(root, &mut v);
        let mut min_res = i32::max_value();
        for i in 1..v.len() {
            if (v[i] - v[i-1]).abs() < min_res {
                min_res = (v[i] - v[i-1]).abs();
            }
        }
        return min_res;
    }

    fn mid_traversal(node : Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }

        // left
        let mut n_b = node.as_ref().unwrap().borrow_mut();
        let (l,r) = (n_b.left.take(), n_b.right.take());
        Solution::mid_traversal(l, v);
        // this
        v.push(n_b.val);
        // right
        Solution::mid_traversal(r, v);
    }
}
fn main() {
    println!("Hello, world!");
}
