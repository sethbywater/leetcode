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

	/// For testing, mostly
	/// Returns an iterator which iterates throug `TreeNode`s in the order:
	/// parent, left, right
	pub fn preorder_iter(self) -> PreorderIter {
		PreorderIter {
			stack: vec![Some(Rc::new(RefCell::new(self)))]
		}
	}
}

/// Creates a tree from a vector. Makes setting up tests way easier
pub fn root_from_slice(src: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {	
	fn recurse(src: &[Option<i32>], ptr: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
		match src[*ptr] {
			None => None,
			Some(val) => {
				*ptr += 1;
				Some(Rc::new(RefCell::new(TreeNode {
					val,
					left: recurse(src, ptr),
					right: recurse(src, { *ptr += 1; ptr })
				})))
			}
		}
	}

	let mut ptr = 0;
	recurse(src, &mut ptr)
}

pub struct PreorderIter {
	stack: Vec<Option<Rc<RefCell<TreeNode>>>>
}

impl PreorderIter {
	pub fn from_root(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
		PreorderIter {
			stack: vec![root]
		}
	}
}

impl Iterator for PreorderIter {
	type Item = Rc<RefCell<TreeNode>>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(next) = self.stack.pop() {
			if let Some(node) = next {
				self.stack.push(node.borrow().right.clone());
				self.stack.push(node.borrow().left.clone());
				return Some(node.clone())
			}
		}
		None
	}
}

#[test]
fn test_tree_from_slice() {
	let src = [
		Some(20),
		Some(8),
		Some(4),
		None,
		None,
		Some(12),
		Some(10),
		None,
		None,
		Some(14),
		None,
		None,
		None
	];
	let vec: Vec<i32> = 
		PreorderIter::from_root(root_from_slice(&src))
			.map(|x| x.borrow().val)
			.collect();
	
	assert_eq!(vec![20, 8, 4, 12, 10, 14],	vec)
}


	// // Find the start of the right node
				// let mut r = l.clone(); // This value is never used, but must be initialized
				// let mut nones = 2;
				// let n = src.len();
				// for i in l..=n {
				// 	if i == n { panic!("Tree is not terminated") }
				// 	if nones == 0 {
				// 		r = i;
				// 		break
				// 	}
				// 	match src[i] {
				// 		None => nones -= 1,
				// 		Some(_) => nones += 1
				// 	}
				// }
				// Some(Rc::new(RefCell::new(TreeNode {
				// 	val,
				// 	left: recurse(src, l),
				// 	right: recurse(src, r)
				// })))