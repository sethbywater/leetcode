//! Structures used by Leetcode in problems

use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>
}

impl ListNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		ListNode {
			val,
			next: None
		}
	}

	/// Added by me, not Leetcode, although I use it in solutions
	pub fn push(&mut self, val: i32) {
		match &mut self.next {
			Some(node) => node.push(val),
			None => self.next = Some(Box::new(ListNode::new(val)))
		}
	}
}

/// This is the structure that Leetcode uses for trees
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