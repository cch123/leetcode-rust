// Definition for a binary tree node.
#![feature(nll)]
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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (&root1, &root2) {
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (None, None) => return true,
            (Some(r1), Some(r2)) => {
                // 那么就要左左比较
                // 否则要右右比较
                if r1.borrow().val != r2.borrow().val {
                    return false;
                }

                // 不用 take 怎么都过不了编译
                // 不信你试试
                let mut r1_b = r1.borrow_mut();
                let mut r2_b = r2.borrow_mut();
                let (r1_l, r1_r, r2_l, r2_r) = (
                    r1_b.left.take(),
                    r1_b.right.take(),
                    r2_b.left.take(),
                    r2_b.right.take(),
                );

                if r1_l.is_none() && r2_l.is_none() {
                    /*
                            O                     O
                           /  \                  /  \
                        None  Some/None        None Some/None
                    */
                    return Solution::flip_equiv(r1_l, r2_l) && Solution::flip_equiv(r1_r, r2_r);
                }

                if r1_l.is_some() && r2_l.is_some() {
                    /*
                            O                     O
                           /  \                  /  \
                        Some  Some/None        Some Some/None
                    */
                    // 比较 left 和 left
                    if r1_l.as_ref().unwrap().borrow().val == r2_l.as_ref().unwrap().borrow().val {
                        return Solution::flip_equiv(r1_l, r2_l) && Solution::flip_equiv(r1_r, r2_r);
                    }
                }

                // 除了上面的两种情况，其它情况应该都是左右比较，右左比较
                return Solution::flip_equiv(r1_l, r2_r) && Solution::flip_equiv(r1_r, r2_l);
            }
        }
    }
}

fn main() {
    let r1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let r2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    {
        let x = r1.as_ref().unwrap();
        x.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    }
    {
        let x = r2.as_ref().unwrap();
        x.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    }
    {
        println!("{:?}, {:?}", r1, r2);
    }
    println!("{}", Solution::flip_equiv(r1, r2));
}
