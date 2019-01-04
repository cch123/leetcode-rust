struct Solution;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        Solution::traverse(root, &mut max_depth, 0);
        return max_depth;
    }

    fn traverse(n:Option<Rc<RefCell<TreeNode>>>, max_depth : &mut i32, parent_depth : i32) {
        if n.is_none() {
            return
        }

        if parent_depth+1 > *max_depth {
            *max_depth = parent_depth +1;
        }

        let mut n_b = n.as_ref().unwrap().borrow_mut();
        let (mut l, mut r) = (n_b.left.clone(), n_b.right.clone());
        Solution::traverse(l,  max_depth, parent_depth+1);
        Solution::traverse(r,  max_depth, parent_depth+1);
    }
}

fn main() {
    println!("Hello, world!");
}


/*
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_depth_ref(&root)
    }
}

pub fn max_depth_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            max(
                max_depth_ref(&node.borrow().left),
                max_depth_ref(&node.borrow().right),
            ) + 1
        }
        None => 0,
    }
}
*/