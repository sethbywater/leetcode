//! A array is a mountain array if there is an index i before which elements
//! only increase and after which elements only decrease
//! 
//! Given an array, find the length of the longest mountain subarray, or 0
//! if there is no mountain

use std::cmp::Ordering::{Greater, Less, Equal};

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
	let n = arr.len();
	if n < 3 { return 0 }
	let mut last = Equal;
	let mut cur = 0;
	let mut best = 0;
	let mut up = false;
	for i in 1..n {
		let here = arr[i].cmp(&arr[i-1]);
		match (here, last) {
			(Greater, Greater) => cur += 1,
			(Less, Less) => if up { cur += 1; best = best.max(cur) },
			(Less, Greater) => { cur += 1; best = best.max(cur) }
			(Greater, Less) | (Greater, Equal) => { cur = 2; up = true },
			(Equal, _) => { cur = 0; up = false },
			(Less, Equal) => cur = 0,
		}
		last = here;
	}
	best
}