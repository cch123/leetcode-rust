// Definition for a binary tree node.
struct Solution;
#[macro_export]
macro_rules! btree {
    () => {
        None
    };
    ($($e:tt), *) => {
        {
            let elems = $crate::null_to_none![$($e), *];
            let head = Some($crate::Rc::new($crate::RefCell::new($crate::TreeNode::new(elems[0].unwrap()))));
            let mut nodes = std::collections::VecDeque::new();
            nodes.push_back(head.as_ref().unwrap().clone());

            for i in elems[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(val) = i[0]{
                    node.borrow_mut().left = Some($crate::Rc::new($crate::RefCell::new($crate::TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if i.len() > 1 {
                    if let Some(val) = i[1] {
                        node.borrow_mut().right = Some($crate::Rc::new($crate::RefCell::new($crate::TreeNode::new(val))));
                        nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                    }
                }
            }
            head
        }
    };
}
#[macro_export]
macro_rules! null_to_none {
    (@start $($e:tt), *) => {
        {
            let mut ret: Vec<Option<i32>> = vec![];
            $crate::null_to_none![@next ret; $($e), *];
            ret
        }
    };
    (@next $vec:expr; null, $($tail:tt), *) => {
        $vec.push(None);
        $crate::null_to_none![@next $vec; $($tail), *];
    };
    (@next $vec:expr; $e:tt, $($tail:tt), *) => {
        $vec.push(Some($e));
        $crate::null_to_none![@next $vec; $($tail), *];
    };
    (@next $vec:expr; null) => {
        $vec.push(None);
    };
    (@next $vec:expr; $e:tt) => {
        $vec.push(Some($e));
    };
    ($($e:tt), *) => {
        $crate::null_to_none![@start $($e), *]
    };
}

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
    fn print(v : &Vec<Option<Rc<RefCell<TreeNode>>>>) {
        for elem in v {
            if elem.is_some() {
                print!(" {}", elem.as_ref().unwrap().borrow_mut().val);
            }else {
                print!(" none");
            }
        }
        println!("");
    }
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 层序遍历就行了
        let mut cur_level = vec![root];
        let mut res = 0;
        loop {
            let (mut some_start, mut some_end) = (-1, -1);
            let mut next_level = vec![];
            Solution::print(&cur_level);
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
    use crate::btree;
    use crate::TreeNode;
    let r = btree![1,1,1,1,1,1,1,null,null,null,1,null,null,null,null,2,2,2,2,2,2,2,null,2,null,null,2,null,2];
    println!("{}", Solution::width_of_binary_tree(r));
}
