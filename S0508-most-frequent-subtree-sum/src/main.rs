// Definition for a binary tree node.
pub use leetcode_prelude::*;

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
struct Solution;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn traverse_tree_and_get_sum(root: Option<Rc<RefCell<TreeNode>>>, val_counter: &mut HashMap<i32, i32>) -> i32 {
        match root {
            Some(r) => {
                let mut root_b = r.as_ref().borrow_mut();
                let l_val = Solution::traverse_tree_and_get_sum(root_b.left.take(), val_counter);
                let r_val = Solution::traverse_tree_and_get_sum(root_b.right.take(), val_counter);
                let res = root_b.val + l_val + r_val;
                let res_cnt = val_counter.get(&res).unwrap_or(&0) + 1;
                val_counter.insert(res,res_cnt);
                return res;
            },
            None => return 0,
        }
    }

    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut val_count = HashMap::new();
        Solution::traverse_tree_and_get_sum(root, &mut val_count);
        let mut max_cnt = 0;
        let mut res = vec![];
        for (k,v) in val_count.iter() {
            if *v > max_cnt {
                max_cnt = *v;
                res = vec![];
                res.push(*k);
                continue
            }
            if *v == max_cnt {
                res.push(*k);
                continue
            }
        }
        return res;
    }
}

fn main() {
    println!("{:?}", Solution::find_frequent_tree_sum(btree![5,2,3]));
}
