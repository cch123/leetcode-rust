struct Solution;
use leetcode_prelude::*;
// Definition for a binary tree node.

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let (mut cnt, mut res) = (0,0);
        Solution::mid_order(root,k, &mut cnt, &mut res);
        return res;
    }

    fn mid_order(n: Option<Rc<RefCell<TreeNode>>>, k:i32, cnt: &mut i32, res: &mut i32) {
        if n.is_none() {
            return
        }

        let mut n_b = n.as_ref().unwrap().borrow_mut();


        Solution::mid_order(n_b.left.take(), k, cnt, res);

        *cnt += 1;
        if *cnt == k {
            *res = n_b.val;
            return
        }

        Solution::mid_order(n_b.right.take(), k, cnt, res);
    }
}

fn main() {
    let r = btree![3,1,4,null,2];
    println!("{:?}", Solution::kth_smallest(r,1));
}
