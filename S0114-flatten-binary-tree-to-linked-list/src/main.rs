struct Solution;
use leetcode_prelude::*;

// Definition for a binary tree node.
//#[derive(Debug, PartialEq, Eq)]
//pub struct TreeNode {
//  pub val: i32,
//  pub left: Option<Rc<RefCell<TreeNode>>>,
//  pub right: Option<Rc<RefCell<TreeNode>>>,
//}
//
//impl TreeNode {
//  #[inline]
//  pub fn new(val: i32) -> Self {
//    TreeNode {
//      val,
//      left: None,
//      right: None
//    }
//  }
//}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        };
        Solution::flatten(&mut root.as_ref().unwrap().borrow_mut().left);
        Solution::flatten(&mut root.as_ref().unwrap().borrow_mut().right);

        let mut left = root.as_ref().unwrap().borrow_mut().left.take();
        let mut right = root.as_ref().unwrap().borrow_mut().right.take();

        match (left.as_ref(), right.as_ref()) {
            (None, _) => {
                // take 消耗了 option，所以要赋值回去
                root.as_ref().unwrap().borrow_mut().right = right;
                return;
            }
            (_, None) => {
                root.as_ref().unwrap().borrow_mut().right = Some(Rc::clone(&left.unwrap()));
                return;
            }
            (Some(_), Some(_)) => {
                let (mut left, mut right) = (left.unwrap(), right.unwrap());
                root.as_ref().unwrap().borrow_mut().right = Some(Rc::clone(&left));
                // 这里也可以不要 cursor 的
                let mut cursor = left;
                loop {
                    let mut next;
                    match cursor.borrow_mut().right {
                        Some(ref node) => {
                            next = Rc::clone(node);
                        }
                        None => {
                            break;
                        }
                    }
                    cursor = next;
                }

                cursor.borrow_mut().right = Some(Rc::clone(&right));
            }
        }
    }
}

fn main() {
    let mut l = btree![1, 2, 5, 3, 4, null, 6];
    println!("{:?}", l);
    Solution::flatten(&mut l);
    println!("{:?}", l);

    let mut l = btree![1, 2];
    println!("{:?}", l);
    Solution::flatten(&mut l);
    println!("{:?}", l);
}
