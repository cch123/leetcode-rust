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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 层序遍历
        let mut cur_level = vec![root];
        loop {
            let first_node_of_this_line = cur_level[0].as_ref().unwrap().borrow().val;
            let mut next_level = vec![];
            for n in cur_level {
                let mut n_b = n.as_ref().unwrap().borrow_mut();
                if n_b.left.is_some() {
                    next_level.push(n_b.left.take());
                }
                if n_b.right.is_some() {
                    next_level.push(n_b.right.take());
                }
            }
            if next_level.len() == 0 {
                return first_node_of_this_line;
            }
            cur_level = next_level;
        }
    }
}

fn main() {
    let r = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let x = r.as_ref().unwrap();
    {
        x.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    }
    {
        x.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    }
    println!("{:?}", r);

    println!("{}", Solution::find_bottom_left_value(r));
}
