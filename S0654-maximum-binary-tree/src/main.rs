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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        // find the max index
        let (mut max_val, mut max_val_idx) = (nums[0], 0);
        for (idx, elem) in nums.iter().enumerate() {
            if *elem > max_val {
                max_val = *elem;
                max_val_idx = idx;
            }
        }
        //println!("max_val: {:?}, nums: {:?}", max_val, nums);

        // split arr
        let (left_vec, right_vec) = (
            nums[0..max_val_idx].to_vec(),
            nums[max_val_idx + 1..].to_vec(),
        );
        let (left_tree, right_tree) = (
            Solution::construct_maximum_binary_tree(left_vec),
            Solution::construct_maximum_binary_tree(right_vec),
        );

        let root = Some(Rc::new(RefCell::new(TreeNode::new(max_val))));
        root.as_ref().unwrap().borrow_mut().left = left_tree;
        root.as_ref().unwrap().borrow_mut().right = right_tree;
        return root;
    }
}

fn main() {
    /*
    let mut v = vec![1,2,34,4];
    let mut x = &v[1..3].to_vec();
    println!("{:?}, {:?}",x,v);
    let mut z = v.split_at(2);
    println!("{:?}, {:?}",z,v);
    let zz = &v[4..].to_vec();
    println!("{:?}, {:?}",zz,v);
    */
    let v = vec![1, 5, 3, 2, 1];
    let r = Solution::construct_maximum_binary_tree(v);
    println!("{:?}", r);
}
