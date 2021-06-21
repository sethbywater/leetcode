//! # 53. Maximum subarray
//!
//! Given an array of integers, find the contiguous subarray (with
//! at least one number, and return its sum

pub fn max_subarrary(nums: Vec<i32>) -> i32 {
	let mut max_so_far = std::i32::MIN;
	let mut max_ending_here = 0;
	for n in nums {
		max_ending_here += n;
		max_so_far = max_so_far.max(max_ending_here);
		max_ending_here = max_ending_here.max(0);
	}
	max_so_far
}

#[test]
fn example_1() {
	assert_eq!(
		max_subarrary(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
		6
	)
}

#[test]
fn example_2() {
	assert_eq!(
		max_subarrary(vec![1]),
		1
	)
}

#[test]
fn example_3() {
	assert_eq!(
		max_subarrary(vec![5, 4, -1, 7, 8]),
		23
	)
}