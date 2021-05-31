//! # 42. Trapping rain water
//!
//! Given n non-negative integers representing an elevation map where
//! the width of eachbar is 1, compute how much water it can trap
//! after raining

pub fn trap(height: Vec<i32>) -> i32 {
	// Find the height of the water at each point
	// This is equal to the minimum of the last highest
	// and the next highest, minus the current elevation
	let n = height.len();
	if n < 3 { return 0 }
	let mut l = vec![height[0]; n];	// There is a way to make these one array
	let mut r = vec![height[n-1]; n];
	for i in 1..n-1 {
		l[i] = l[i-1].max(height[i]);
		r[n-i-1] = r[n-i].max(height[n-i-1]);
	}
	let mut w = 0;
	for i in 0..n {
		w += (l[i].min(r[i]) - height[i]).max(0)
	}
	w
}