//! # 503. Next greater element II
//! 
//! Given a circular integer array nums (i.e., the next element of nums[nums.length
//! - 1] is nums[0]), return ~the~ ~~~next greater number~~~ ~for every element in 
//! nums.~
//! 
//! The next greater number of a number is the first greater number to its traversing
//! -order next in the array, which means you could search circularly to find its next 
//! greater number. If it doesn't exist, return -1 for this number.

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
	let n = nums.len();
	let mut res = vec![-1; n];
	let mut stack: Vec<usize> = Vec::with_capacity(n);
	for i in 0..n*2 {
		let cur = nums[i % n];
		while !stack.is_empty() && cur > nums[*stack.last().unwrap()] {
			let pop = stack.pop().unwrap();
			res[pop] = cur;
		}
		if i < n { stack.push(i) }
	}
	res
}
