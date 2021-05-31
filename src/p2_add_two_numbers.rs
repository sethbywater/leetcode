//! # 2. Add two numbers
//! Given two linked lists representing two numbers, with each node representing
//! one digit and digits stored in reverse order, multiply the two numbers and
//!return the product as a linked list

use crate::collections::ListNode;

/// Top-level function to call the helper function
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	add_three_numbers(l1, l2, 0)
}

/// Recursive function which also considers the remainder from adding previous digits
fn add_three_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
	Some(Box::new( match (l1, l2, carry) {
		(None, None, 0) => return None,
		(None, None, n) => ListNode::new(n),
		(a, b, _n) => ListNode {
			next: add_three_numbers(
				a.and_then(|x| { carry += x.val; x.next }),
				b.and_then(|x| { carry += x.val; x.next }),
				carry / 10),
			val: carry % 10
		}
	} ))
}