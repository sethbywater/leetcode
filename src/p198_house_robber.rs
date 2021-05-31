//! # 198. House robber
//! 
//! You are a professional robber planning to rob houses
//! along a street. Each house has a certain amount of
//! money stashed, the only constraint stopping you from
//! robbing each of them is that adjacent houses have security
//! systems connected and ~~it will automatically contact
//! the police if two adjacent houses were broken into in
//! the same night.~~
//! 
//! Given an integer array representing the amount of money
//! in each house, return ~the maximum amount of money you can
//! rob tonight~ ~~~without alerting the police.~~~

use std::cmp;

pub fn rob(mut nums: Vec<i32>) -> i32 {
	let l = nums.len();
	if l >= 2 {
		nums[l-2] = nums[l-2].max(nums[l-1]);
		for i in (0..l-2).rev() {
			nums[i] = cmp::max(
				nums[i+2] + nums[i],
				nums[i+1]
			)
		}
	}
	nums.into_iter().max().unwrap()
}