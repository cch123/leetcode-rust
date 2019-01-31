struct Solution;
use leetcode_prelude::*;

// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Solution::dfs(root, &mut res);
        res
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res:&mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let r_u_b = root.as_ref().unwrap().borrow_mut();
        let left_step_cnt = Solution::dfs(r_u_b.left.clone(), res);
        let right_step_cnt = Solution::dfs(r_u_b.right.clone(), res);
        //println!("{}, {}",left_step_cnt, right_step_cnt);
        *res += left_step_cnt.abs() + right_step_cnt.abs();

        r_u_b.val - 1 + left_step_cnt + right_step_cnt
    }
}

fn main() {
    let r = btree![3,0,0];
    //println!("{}", Solution::distribute_coins(r));
    println!("{}", Solution::distribute_coins(btree![1,0,0,null,3]));
}
