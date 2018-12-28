// Definition for a binary tree node.
struct Solution;

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
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 层序遍历就行了
        let mut cur_level = vec![root];
        let mut res = 0;
        loop {
            let (mut some_start, mut some_end) = (-1, -1);
            let mut next_level = vec![];
            for (idx, n) in cur_level.iter().enumerate() {
                if some_start == -1 && n.is_some() {
                    some_start = idx as i32;
                }
                if some_start != -1 && n.is_some() {
                    some_end = idx as i32;
                }
                match n {
                    Some(elem) => {
                        let mut e_b = elem.borrow_mut();
                        let (l, r) = (e_b.left.take(), e_b.right.take());
                        next_level.append(&mut vec![l, r]);
                    }
                    None => {
                        next_level.append(&mut vec![None, None]);
                    }
                }
                //println!("start{},end{}", some_start, some_end);
                match (some_start, some_end) {
                    (-1, -1) => return res,
                    (x, y) => {
                        if res < (y - x + 1) {
                            res = y - x + 1;
                        }
                    }
                }
            }
            cur_level = next_level;
        }
    }
}
fn main() {
    let r = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    r.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    r.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    r.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    r.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    r.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    r.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("{}", Solution::width_of_binary_tree(r));
}
