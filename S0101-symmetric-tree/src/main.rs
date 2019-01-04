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
    fn symmetric(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (l.as_ref(), r.as_ref()) {
            (None, None) => return true,
            (Some(x), Some(y)) => {
                if x.borrow_mut().val != y.borrow_mut().val {
                    return false;
                }
                let (mut x_b, mut y_b) = (x.borrow_mut(), y.borrow_mut());
                // 左 = 右，右 = 左
                return Solution::symmetric(
                    x_b.left.take(),
                    y_b.right.take(),
                ) && Solution::symmetric(
                    x_b.right.take(),
                    y_b.left.take(),
                );
            }
            (_, _) => return false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut r_b = root.as_ref().unwrap().borrow_mut();
        let (l, r) = (r_b.left.take(), r_b.right.take());
        return Solution::symmetric(l, r);
    }
}
fn main() {
    let l = btree![1,2,2,3,4,4,3];
    println!("{}", Solution::is_symmetric(l));
}


/*
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(root.clone(), root)
    }

    fn is_mirror(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (Some(t1), Some(t2)) => {
                t1.borrow().val == t2.borrow().val
                    && Self::is_mirror(t1.borrow().left.clone(), t2.borrow().right.clone())
                    && Self::is_mirror(t1.borrow().right.clone(), t2.borrow().left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
*/