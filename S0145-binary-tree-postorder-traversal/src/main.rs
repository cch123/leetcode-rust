fn main() {
    println!("Hello, world!");
}

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
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Solution::traverse(root, &mut |x: i32| res.push(x));
        res
    }

    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>, consumer: &mut impl FnMut(i32)) {
        if root.is_none() {
            return;
        }
        let r_b = root.as_ref().unwrap().borrow_mut();

        Solution::traverse(r_b.left.clone(), consumer);
        Solution::traverse(r_b.right.clone(), consumer);
        consumer(r_b.val);
    }
}
