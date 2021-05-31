//! # 986. Binary tree cameras
//!
//! Given the root of a binary tree, you may install cameras a any node
//! to monitor all adjacent nodes. Return the minimum number of cameras
//! required to monitor all nodes

use std::{cell::RefCell, rc::Rc};
use crate::collections::TreeNode;

pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	// Start from one above the root, in case it needs a camera itself
	let head = Some(Rc::new(RefCell::new(TreeNode { val: 0, left: None, right: root })));

	fn helper(root: &Option<Rc<RefCell<TreeNode>>>, cams: &mut i32) -> Cam {
		use Cam::{IsCamera, NeedCamera, NotReq};
		match root {
			Some(node) => {
				let (l, r) = (helper(&node.borrow().left, cams), helper(&node.borrow().right, cams));
				match (l, r) {
					(NeedCamera, _) | (_, NeedCamera) => { *cams += 1; IsCamera },
					(IsCamera, _) | (_, IsCamera) => NotReq,
					_ => NeedCamera,
				}
			}
			None => NotReq
		}
	}

	let mut cams = 0;
	helper(&head, &mut cams);
	cams
}

enum Cam {
	IsCamera,
	NeedCamera,
	NotReq
}