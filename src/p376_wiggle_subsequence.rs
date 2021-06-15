//! # 376. Wiggle subsequence
//! 
//! A wiggle sequence is a sequence of numbers in which the difference
//! between adjacent numbers strictly alternates between positive
//! and negative.
//! 
//! A subsequence is obtained by deleting chosen elements (possibly none)
//! from the original sequence, leaving the others in their original order
//!
//! Return the longest wiggle subsequence given a sequence of numbers

use std::cmp::Ordering;

pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
	let n = nums.len();
	if n < 2 { return n as i32 }
	
	let mut up_going_up = true;
	let mut down_going_up = false;

	let mut start_up = 1;
	let mut start_down = 1;

	for i in 1..n {
		match nums[i].cmp(&nums[i-1]) {
			Ordering::Greater => {
				if up_going_up { up_going_up = false; start_up += 1 }
				if down_going_up { down_going_up = false; start_down += 1 }
			}
			Ordering::Less => {
				if !up_going_up { up_going_up = true; start_up += 1 }
				if !down_going_up { down_going_up = true; start_down += 1 }
			}
			_ => continue
		}
	}

	start_up.max(start_down)
}

#[test]
fn testing() {
	assert_eq!(wiggle_max_length(vec![1,7,4,9,2,5]), 6)
}

#[test]
fn testing_1() {
	assert_eq!(wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]), 7)
}

#[test]
fn testing_2() {
	assert_eq!(wiggle_max_length(vec![1,2,3,4,5,6,7,8,9]), 2)
}