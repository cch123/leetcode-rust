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

type Node = Option<Rc<RefCell<TreeNode>>>;

use std::cell::RefCell;
use std::rc::Rc;

/*
学习大佬的写法
impl Solution {
    pub fn flatten(root: &mut Node) {
        fn core(node: &mut Node, mut next: Node) -> Node {
            if let Some(node) = node {
                next = core(&mut node.borrow_mut().right, next);
                next = core(&mut node.borrow_mut().left, next);
                node.borrow_mut().right = next;
                node.borrow_mut().left = None;
                next = Some(node.clone());
            }
            next
        }
        core(root, None);
    }
}
*/

impl Solution {
    pub fn flatten(root: &mut Node) {
        if root.is_none() {
            return;
        };

        let mut r = root.as_ref().unwrap().borrow_mut();
        Solution::flatten(&mut r.left);
        Solution::flatten(&mut r.right);

        let left = r.left.take();
        let right = r.right.take();

        match (left.as_ref(), right.as_ref()) {
            (None, _) => {
                // take 消耗了 option，所以要赋值回去
                r.right = right;
                return;
            }
            (_, None) => {
                r.right = Some(Rc::clone(&left.unwrap()));
                return;
            }
            (Some(_), Some(_)) => {
                let (left, right) = (left.unwrap(), right.unwrap());
                r.right = Some(Rc::clone(&left));
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
