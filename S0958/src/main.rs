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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        //let root = root.unwrap();
        //let mut n = root.borrow_mut();

        let mut cur_level:Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];

        let mut cur_level_len= 1;
        loop {
            if cur_level.len() != cur_level_len {
                return false
            }

            let mut next_level:Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
            let l = cur_level.len();
            let mut idx = 0 ;

            while idx < l {
                let mut elem = &cur_level[idx].as_ref().unwrap().borrow_mut().left ;
                //next_level.push(elem.borrow_mut().left);
                next_level.push(*elem);
            }

            return true;
        }

        return false;
    }
}

fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    Solution::is_complete_tree(root);
}
