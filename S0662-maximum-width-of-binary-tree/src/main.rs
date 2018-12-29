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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 层序遍历就行了
        let (mut cur_level, mut cur_level_idx) = (vec![root], vec![0]);
        let mut max_width = 0;
        loop {
            let (mut some_start, mut some_end) = (-1, -1);
            let (mut next_level, mut next_level_idx) = (vec![], vec![]);

            if cur_level.len() == 0 {
                return max_width;
            }

            for (idx, n) in cur_level.iter().enumerate() {
                let cur_node_idx = cur_level_idx[idx];

                if some_start == -1 && n.is_some() {
                    some_start = cur_node_idx;
                }
                if some_start != -1 && n.is_some() {
                    some_end = cur_node_idx;
                }

                // 当前节点是 some
                // 需要把孩子节点和其索引推到下一层去
                if let Some(elem) = n {
                    let mut e_b = elem.borrow_mut();
                    let (l, r) = (e_b.left.take(), e_b.right.take());

                    if l.is_some() {
                        next_level.push(l);
                        next_level_idx.push(cur_node_idx * 2);
                    }

                    if r.is_some() {
                        next_level.push(r);
                        next_level_idx.push(cur_node_idx * 2 + 1);
                    }
                }

                if max_width < (some_end - some_start + 1) {
                    max_width = some_end - some_start + 1;
                }
            }


            // 下一层初始化
            cur_level = next_level;
            cur_level_idx = next_level_idx;
        }
    }
}

fn main() {
    use crate::btree;
    use crate::TreeNode;
    let r = btree![
        1, 1, 1, 1, 1, 1, 1, null, null, null, 1, null, null, null, null, 2, 2, 2, 2, 2, 2, 2,
        null, 2, null, null, 2, null, 2
    ];
    println!("{}", Solution::width_of_binary_tree(r));
}
