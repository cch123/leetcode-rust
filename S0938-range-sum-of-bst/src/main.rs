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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut r_b = root.as_ref().unwrap().borrow_mut();
        if r_b.val > r {
            return Solution::range_sum_bst(r_b.left.take(), l, r);
        }
        if r_b.val < l {
            return Solution::range_sum_bst(r_b.right.take(), l, r);
        }
        return r_b.val
            + Solution::range_sum_bst(r_b.left.take(), l, r)
            + Solution::range_sum_bst(r_b.right.take(), l, r);
    }
}

fn main() {
    println!("Hello, world!");
}
