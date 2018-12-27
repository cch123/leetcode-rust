// 递归能写出来就不错了。。。。
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            let res_vec: Vec<i32> = vec![];
            return res_vec;
        }
        let val;
        let mut left_vec: Vec<i32> = vec![];
        let mut right_vec: Vec<i32> = vec![];

        let root_ptr = root.as_ref().unwrap().borrow_mut();
        val = root_ptr.val;

        if let Some(l) = &root_ptr.left {
            left_vec = Solution::inorder_traversal(Some(Rc::clone(l)));
        }
        if let Some(r) = &root_ptr.right {
            right_vec = Solution::inorder_traversal(Some(Rc::clone(r)));
        }

        // 把 borrow 的还回去
        drop(root_ptr);

        // 左子树结果 + cur 节点值 + 右子树结果
        left_vec.push(val);
        left_vec.append(&mut right_vec);
        println!("{:?}", left_vec);
        println!("{:?}", root);
        return left_vec;
    }
}

fn main() {
    let mut r = TreeNode::new(1);
    r.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    r.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let root = Some(Rc::new(RefCell::new(r)));
    println!("{:?}", root);
    Solution::inorder_traversal(root);
}
