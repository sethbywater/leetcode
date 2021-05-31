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