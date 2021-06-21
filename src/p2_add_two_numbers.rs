//! # 2. Add two numbers
//! Given two linked lists representing two numbers, with each node representing
//! one digit and digits stored in reverse order, multiply the two numbers and
//!return the product as a linked list

#[allow(unused_imports)] // Imported for testing but still throws a warning
use crate::collections::{ListNode, list_root_to_vec};

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

#[test]
fn example_1() {
	// 342 + 465 = 807
	let l1 = ListNode::root_from_slice(&[2, 4, 3]);
	let l2 = ListNode::root_from_slice(&[5, 6, 4]);
	let correct = ListNode::root_from_slice(&[7, 0, 8]);
	assert_eq!(
		list_root_to_vec(add_two_numbers(l1, l2)),
		list_root_to_vec(correct)
	)
}

#[test]
fn example_2() {
	let l1 = ListNode::root_from_slice(&[0]);
	let l2 = ListNode::root_from_slice(&[0]);
	let correct = ListNode::root_from_slice(&[0]);
	assert_eq!(
		list_root_to_vec(add_two_numbers(l1, l2)),
		list_root_to_vec(correct)
	)
}

#[test]
fn example_3() {
	// 342 + 465 = 807
	let l1 = ListNode::root_from_slice(&[9, 9, 9, 9, 9, 9, 9]);
	let l2 = ListNode::root_from_slice(&[9, 9, 9, 9]);
	let correct = ListNode::root_from_slice(&[8, 9, 9, 9, 0, 0, 0, 1]);
	assert_eq!(
		list_root_to_vec(add_two_numbers(l1, l2)),
		list_root_to_vec(correct)
	)
}
