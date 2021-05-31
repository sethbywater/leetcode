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
    if let Some(node) = &root {
        let (u, nu) = rob_iii_helper(node);
        return u.max(nu)
    }
    else { return 0 }
}

fn rob_iii_helper(cur: &Rc<RefCell<TreeNode>>) -> (i32, i32) {
    let mut l = (0, 0);
    let mut r = (0, 0);
    if let Some(node) = &cur.borrow().left {
        l = rob_iii_helper(node)
    }
    if let Some(node) = &cur.borrow().right {
        r = rob_iii_helper(node)
    }
    (l.1 + r.1, (l.0 + r.0 + cur.borrow().val).max(l.1 + r.1))
}