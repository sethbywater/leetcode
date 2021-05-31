//! # 496. Next greater element I
//! 
//! You are given two integer arrays nums1 and nums2 both of ~~unique~~
//! elements, where nums1 is a subset of nums2.
//! 
//! Find all the next greater numbers for num1's elements in the corresponding
//! places of nums2.
//! 
//! The next greater number of a number in nums1 is the first greater number
//! to its right in nums2. If it does not exist, return -1 for this number.

use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
	let mut map: HashMap<i32, i32> = HashMap::new();
	let mut stack: Vec<i32> = Vec::with_capacity(nums2.len());
	for n in nums2 {
		while !stack.is_empty() && n > *stack.last().unwrap() {
			let pop = stack.pop().unwrap();
			map.insert(pop, n);
		}
		stack.push(n)
	}
	nums1.into_iter().map(|n| *map.get(&n).unwrap_or(&-1)).collect()
}