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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // mid order traversal
        let mut v = vec![];
        Solution::traversal(root, &mut v);
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cursor = dummy.clone();
        for num in v {
            let mut next = Rc::new(RefCell::new(TreeNode::new(num)));
            cursor.borrow_mut().right.replace(next.clone());
            cursor = next.clone();
        }
        return dummy.borrow_mut().right.take();
    }

    fn traversal(n:Option<Rc<RefCell<TreeNode>>>,v : &mut Vec<i32>) {
        if n.is_none() {
            return;
        }

        let mut n_b = n.as_ref().unwrap().borrow_mut();
        Solution::traversal(n_b.left.take(), v);
        v.push(n_b.val);
        Solution::traversal(n_b.right.take(),v);
    }
}

fn main() {
    let r = btree![5,3,6,2,4,null,8,1,null,null,null,7,9];
    let x = Solution::increasing_bst(r);
    println!("{:?}", x);
}
