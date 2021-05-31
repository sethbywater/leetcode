//! Problems from Leetcode
//! Sorted generally in order as listed by Leetcode, although
//! related questions (i.e. Trapping Rain Water, Trapping Rain
//! Water II) are put next to each other

use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::rc::Rc;
use crate::collections::TreeNode;
use crate::collections::ListNode;
use std::collections::HashMap;
use std::cmp;

/// # 2. Add two numbers
/// Given two linked lists representing two numbers, with each node representing
/// one digit and digits stored in reverse order, multiply the two numbers and
// return the product as a linked list
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	add_three_numbers(l1, l2, 0)
}

fn add_three_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
	Some(Box::new( match (l1, l2, carry) {
		(None, None, 0) => return None,
		(None, None, n) => ListNode::new(n),
		(a, b, _n) => ListNode {
			next: add_three_numbers(
				a.and_then(|x| { carry += x.val; x.next }),
				b.and_then(|x| { carry += x.val; x.next }),
				carry / 10),
			val: carry % 10
		}
	} ))
}

/// # 12. Integer to Roman
/// Convert a given positive integer between 1 and 3999 into roman numerals
pub fn int_to_roman(num: i32) -> String {
	let mut n = num;
	let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
	let strs = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
	let mut s = String::new();
	for i in 0..13 {
		while n >= vals[i] {
			n -= vals[i];
			s.push_str(strs[i]);
		}
	}
	s
}

/// # 42. Trapping rain water
/// ## Given n non-negative integers representing an elevation map where
/// ## the width of eachbar is 1, compute how much water it can trap
/// ## after raining
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

/// # 407. Trapping rain water II
/// ## Given an m * n integer matrix height_map representing the height of
/// ## each unit cell in a 2D elevation map, return the volume of water
/// ## it can trap after raining
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

/// # 53. Maximum subarray
/// ## Given an array of integers, find the contiguous subarray (with
/// ## at least one number, and return its sum
/// Basic Kadane's algorithm
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

/// # 84. Largest rectangle in histogram
/// TODO wrong answer
pub fn largest_rectangle(heights: Vec<i32>) -> i32 {
	let mut best: Vec<i32> = vec![0; heights.len()];
	let mut left: i32;
	let mut right: i32;
	for i in 0..heights.len() {
		left = 0;
		right = 0;
		for j in (0..i).rev() {
			if heights[j] < heights[i] { break }
			left += 1;
		}
		for j in i..heights.len() {
			if heights[j] < heights[i] { break }
			right += 1;
		}
		best[i] = heights[i] * (right + left);
	}
	*best.iter().max().unwrap()
}

/// # 121. Max profit
/// ## Given an array indicating the price of a stock on a particular day,
/// ## return the maximum profit you can acheive by buying on one day and
/// ## selling on a future day
/// Returns the sum of the subarray with the largest possible sum
pub fn max_profit(v: Vec<i32>) -> i32 {
    let mut min_seen = i32::MAX;
    let mut max_profit = 0;
    for num in v {
        max_profit = cmp::max(max_profit, num - min_seen);
        min_seen = cmp::min(min_seen, num);
    }
    max_profit
}

// TODO robbers don't need to take max, just last or first element

/// # 198. House robber
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

/// # 213. House robber II
pub fn rob_ii(nums: Vec<i32>) -> i32 {
	if nums.len() < 4 { return nums.into_iter().max().unwrap() }
	let n = nums.len();
	cmp::max(
		rob_ii_helper(nums.clone(), 0, n - 1),
		rob_ii_helper(nums.clone(), 1, n)
	)
}

/// House robber II resuses code from House robber I
fn rob_ii_helper(mut  nums: Vec<i32>, st: usize, en: usize) -> i32 {
    nums[en-2] = nums[en-2].max(nums[en-1]);
    for i in (st..en-2).rev() {
        nums[i] = cmp::max(
            nums[i+2] + nums[i],
            nums[i+1]
        )
    }
    nums.into_iter().max().unwrap()
}

pub fn rob_iii(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = &root {
        let (u, nu) = rob_iii_helper(node);
        return u.max(nu)
    }
    else { return 0 }
}

fn rob_iii_helper(cur: &Rc<RefCell<TreeNode>>) -> (i32, i32) {
    let mut l = (0, 0);
    let mut r = (0, 0);
    if let Some(node) = &cur.borrow().left {
        l = rob_iii_helper(node)
    }
    if let Some(node) = &cur.borrow().right {
        r = rob_iii_helper(node)
    }
    (l.1 + r.1, (l.0 + r.0 + cur.borrow().val).max(l.1 + r.1))
}

/// # 496 Next greater element I
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

/// # 503. Next greater element II
/// Take an array of numbers and set each element equal to the next element
/// greater than itself, circularly. If no such element exists, set it
/// equal to -1
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

/// # 867. Transpose matrix
/// ## Given a 2D integer matrix, return the transpose of the matrix
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
	let mut res: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix[0].len()];
	for row in 0..matrix.len() {
		for col in 0..matrix[0].len() {
			res[col][row] = matrix[row][col]
		}
	}
	res
}

/// # 1431. Kids with candies
/// ## Given the array 'candies' and the integer 'extra_candies', where
/// ## 'candies[i]' represents the number of candles that the ith kid has,
/// ## return a boolean array indicating if there is a way for each kind that
/// ## 'extra_candies' may be distributed so that they have the greatest number
/// ## of candies
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
	let highest_stash = candies.iter().max().unwrap();
	let diffs: Vec<i32> = candies.iter().map(|x| *highest_stash - x).collect();
	let sum: i32 = diffs.iter().sum();

	let mut can_have_most_candy = vec![];	
	for dif in diffs {
		can_have_most_candy.push((dif <= extra_candies) || ((dif == 0) && (sum <= extra_candies)))
	}
	can_have_most_candy
}

/// # 1553. Minimum number of days to eat n oranges
pub fn min_days(n: i32) -> i32 {
	if n < 3 { return n };
	let mut mem: HashMap<i32, i32> = HashMap::new();
	min_days_helper(n, &mut mem)
}

fn min_days_helper(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
    if n <= 1 { return 1 }
	if let Some(days) = mem.get(&n) { return *days }
	let min = 1 + (n % 2 + min_days_helper(n / 2, mem)).min(n % 3 + min_days_helper(n / 3, mem));
	mem.insert(n, min);
	min
}

/// # 1626. Best team with no conflicts
pub fn best_team_score(mut scores: Vec<i32>, mut ages: Vec<i32>) -> i32 {
	if scores.len() == 0 { return 0 }
	sort_age_score(&mut scores, &mut ages);
	let mut best = scores.clone();
	let mut hi = 0;
	for i in 0..scores.len() {
		for j in 0..i {
			if scores[j] <= scores[i] { best[i] = best[i].max(best[j] + scores[i]) }
		}
		hi = hi.max(best[i])
	}
	hi
}

fn sort_age_score(scores: &mut Vec<i32>, ages: &mut Vec<i32>) {
	// Sort by age, then score
	for i in 0..ages.len() {
		let mut j = i;
		while j > 0 && (ages[j] < ages[j-1] || (ages[j] == ages[j-1] && scores[j] < scores[j-1])) {
			ages.swap(j, j-1);
			scores.swap(j, j-1);
			j -= 1
		}
	}
}