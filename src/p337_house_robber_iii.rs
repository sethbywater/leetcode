//! # 337. House Robber III
//! 
//! The thief has found himself a new place for his theivery
//! again. There is only only entrance to thise area, called
//! root.
//! 
//! Besides the root, each house has one and only one parent
//! house. After a tour, the smart thief realizes that all
//! houses in this place form a binary tree. It will
//! automatically contact the police if ~~two directly-linked
//! houses were broken into on the same night.~~
//! 
//! Given the root of the binary tree, return ~the maximum
//! amount of money the thief can rob~ ~~~without alerting the
//! police.~~~

use std::{cell::RefCell, rc::Rc};

use crate::collections::TreeNode;

pub fn rob_iii(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recurse(cur: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let mut l = (0, 0);
        let mut r = (0, 0);
        if let Some(node) = cur.borrow().left.clone() {
            l = recurse(node)
        }
        if let Some(node) = cur.borrow().right.clone() {
            r = recurse(node)
        }
        (l.1 + r.1, (l.0 + r.0 + cur.borrow().val).max(l.1 + r.1))
    }

    let (u, nu) = recurse(root.unwrap());
    u.max(nu)
}

#[test]
fn example_1() {
    let root = TreeNode::root_from_slice(&[
        Some(3),
        Some(2),
        Some(3), None, None, // Leaf
        None,
        Some(3),
        None,
        Some(1), None, None // Leaf
    ]);
    assert_eq!(rob_iii(root), 7)
}

#[test]
fn example_2() {
    let root = TreeNode::root_from_slice(&[
        Some(3),
        Some(4),
        Some(1), None, None, // Leaf
        Some(3), None, None, // Leaf
        Some(5),
        None,
        Some(1), None, None // Leaf
    ]);
    assert_eq!(rob_iii(root), 9)
}