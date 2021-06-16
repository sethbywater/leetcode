//! # 62. Unique paths
//!
//! A robot is located at the top-left corner of an m * n grid.
//! The robot can only move down or right at any one time. The
//! robot is trying to reach the bottom-right corner. How many
//! unique paths could the robot take?

use std::collections::HashMap;

pub fn unique_paths(m: i32, n: i32) -> i32 {
	let mut mem: HashMap<(i32, i32), i32> = HashMap::new();

	fn helper(m: i32, n: i32, mem: &mut HashMap<(i32, i32), i32>) -> i32 {
		// base cases
		if m == 0 || n == 0 { return 0 }
		if (m, n) == (1, 1) { return 1 }
		// already found
		if let Some(num) = mem.get(&(m, n)) 
			{ return *num }
		if let Some(num) = mem.get(&(n, m))
			{ return *num }

		let num = helper(m-1, n, mem) + helper(m, n-1, mem);
		mem.insert((m, n), num);
		num
	}

	helper(m, n, &mut mem)
}

#[test]
fn example1() {
	assert_eq!(unique_paths(3, 7), 28)
}

#[test]
fn example2() {
	assert_eq!(unique_paths(3, 2), 3)
}

#[test]
fn example3() {
	assert_eq!(unique_paths(7, 3), 28)
}

#[test]
fn example4() {
	assert_eq!(unique_paths(3, 3), 6)
}