struct Solution;

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
       right: None
     }
   }
 }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (None, None) => return None,
            (None, Some(n2)) => return Some(n2.clone()),
            (Some(n1),None) => return Some(n1.clone()),
            (Some(n1),Some(n2))=> {
                let v = n1.borrow_mut().val + n2.borrow_mut().val;
                let left_tree = Solution::merge_trees(n1.borrow_mut().left.clone(), n2.borrow_mut().left.clone());
                let right_tree= Solution::merge_trees(n1.borrow_mut().right.clone(), n2.borrow_mut().right.clone());
                return Some(Rc::new(RefCell::new(TreeNode{
                    val : v,
                    left : left_tree,
                    right : right_tree,
                })));
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
