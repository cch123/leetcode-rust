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

        let mut cur_level: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];

        loop {
            let mut next_level: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
            let mut next_level_has_some = false;
            for (idx, elem) in cur_level.iter().enumerate() {
                match elem {
                    Some(x) => {
                        if idx > 0 && cur_level[idx-1].is_none() {
                            return false;
                        }
                        let left = x.borrow_mut().left.take();
                        let right = x.borrow_mut().right.take();
                        if left.is_some()||right.is_some() {
                            next_level_has_some = true;
                        }

                        next_level.push(left);
                        next_level.push(right);
                    }
                    None => {
                        if next_level.len() > 0 && next_level_has_some == true{
                            return false;
                        }
                        if idx == cur_level.len()-1 && next_level_has_some == false{
                            return true;
                        }
                    },
                }
            }
            cur_level = next_level;
            //println!("{:?}", next_level);
        }
        return false;
    }
}

fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let res = Solution::is_complete_tree(root);
    println!("{}", res);
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let res = Solution::is_complete_tree(root);
    println!("{}", res);
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.as_ref().unwrap().borrow_mut().left= Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let res = Solution::is_complete_tree(root);
    println!("{}", res);
}
