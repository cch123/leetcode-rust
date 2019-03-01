use leetcode_prelude::*;
fn main() {
    let tree = btree![1,2,3,null,4,null,5];
    println!("{}", Solution::is_cousins(tree, 5, 4));
    let tree = btree![1,2,4,3];
    println!("{}", Solution::is_cousins(tree, 3, 4));
    let tree = btree![1,2,3,null, 4];
    println!("{}", Solution::is_cousins(tree, 3, 2));
}


struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let x_idx = Solution::match_traverse(&root, 0, x);
        let y_idx = Solution::match_traverse(&root, 0, y);
        // 2^n + 0~n-1
        let (min, max)  =  (x_idx.min(y_idx), x_idx.max(y_idx));
        //println!("{},{}", max,min);
        // 说明爹一样，不符合定义
        if max - min == 1 && max % 2 == 0 {
            return false
        }
        // 说明是同一个节点
        if max == min {
            return false
        }

        // 同一层的节点 log2 的取整结果一定是一样的
        return (((max+1) as f32).log2() as i32) == (((min+1) as f32).log2() as i32)
    }

    pub fn match_traverse(node: &Option<Rc<RefCell<TreeNode>>>, idx: i32, match_v: i32) -> i32 {
        if node.is_none() {
            return -1;
        }
        let n_u_b = node.as_ref().unwrap().borrow_mut();
        // match !!
        if n_u_b.val == match_v {
            return idx;
        }

        // 叶子节点
        if n_u_b.left.is_none() && n_u_b.right.is_none() {
            return -1;
        }

        let left_idx= Solution::match_traverse(&n_u_b.left, idx * 2 + 1, match_v);
        let right_idx= Solution::match_traverse(&n_u_b.right, idx * 2 + 2, match_v);

        if left_idx != -1 {
            return left_idx;
        }

        return right_idx;
    }
}
