//! # 407. Trapping rain water II
//!
//! Given an m * n integer matrix height_map representing the height of
//! each unit cell in a 2D elevation map, return ~the volume of water
//! it can trap after raining~

use std::{cmp::Reverse, collections::BinaryHeap};

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
	if (height_map.len() < 3) || (height_map[0].len() < 3) { return 0 }
	let n = height_map.len();
	let m = height_map[0].len();
	let mut visited = vec![vec![false; m]; n];
	let mut water = 0;
	let mut max = 0;
	let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::with_capacity(m * n);

	// Load edges
	for (i, h) in height_map[0].iter().enumerate() {
		heap.push((Reverse(*h), 0, i));
		visited[0][i] = true;
	}
	for i in 1..n-1 {
		heap.push((Reverse(height_map[i][0]), i, 0));
		heap.push((Reverse(height_map[i][m-1]), i, m-1));
		visited[i][0] = true;
		visited[i][m-1] = true;
	}
	for (i, h) in height_map[n-1].iter().enumerate() {
		heap.push((Reverse(*h), n-1, i));
		visited[n-1][i] = true;
	}

	while let Some((_, i, j)) = heap.pop() {
		let h = height_map[i][j];
		max = max.max(h);
		water += max - h;
		let neighbors = vec![(i, j+1), (i, j-1), (i+1, j), (i-1, j)];
		for (k, l) in neighbors {
			if k < n && l < m && !visited[k][l] {
				heap.push((Reverse(height_map[k][l]), k, l));
				visited[k][l] = true;
			}
		}
	}
	water
}