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
struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(x), Some(y)) => {
                if x.borrow().val != y.borrow().val {
                    return false;
                }

                let mut x_b = x.borrow_mut();
                let mut y_b = y.borrow_mut();
                return Solution::is_same_tree(
                    x_b.left.take(),
                    y_b.left.take(),
                ) && Solution::is_same_tree(
                    x_b.right.take(),
                    y_b.right.take(),
                );
            }
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (None, None) => return true,
        }
        return false;
    }
}

fn main() {
    let r1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let r2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    println!("{}", Solution::is_same_tree(r1,r2));
}

/*
// leetcode 上的示例
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
    pub fn is_same_tree(p: &mut Option<Rc<RefCell<TreeNode>>>, q: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() || q.is_none() {
            p.is_none() && q.is_none()
        } else {
            let mut left_subtree = p.as_mut().unwrap().borrow_mut();
            let mut right_subtree = q.as_mut().unwrap().borrow_mut();
            if left_subtree.val != right_subtree.val {
                return false;
            }
            if !Self::is_same_tree(&mut left_subtree.left, &mut right_subtree.left) {
                return false;
            }
            if !Self::is_same_tree(&mut left_subtree.right, &mut right_subtree.right) {
                return false;
            }

            true
        }
    }
}
*/