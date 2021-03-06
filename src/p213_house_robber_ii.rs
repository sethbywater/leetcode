//! # 213. House robber II
//! 
//! You are a professional robber planning to rob houses along a street. 
//! Each house has a certain amount of money stashed. All houses at this 
//! place are ~~arranged in a circle.~~ That means the first house is the 
//! neighbor of the last one. Meanwhile, adjacent houses have a security 
//! system connected, and ~~it will automatically contact the police if two 
//! adjacent houses were broken into on the same night.~~
//! 
//! Given an integer array representing the amount of money in each house,
//! return ~the maximum amount of money you can rob~ ~~~without alerting
//! the police.~~~

use std::cmp;

/// Uses the same algorithm from House Robber I, but tests 
/// it for the array excluding the first element, and then
/// excluding the last element and takes the max of the two
pub fn rob_ii(nums: Vec<i32>) -> i32 {
	if nums.len() < 4 { return nums.into_iter().max().unwrap() }

    fn helper(mut nums: Vec<i32>, st: usize, en: usize) -> i32 {
        let mut max = 0;
        nums[en-2] = nums[en-2].max(nums[en-1]);
        for i in (st..en-2).rev() {
            nums[i] = cmp::max(
                nums[i+2] + nums[i],
                nums[i+1]
            );
            max = max.max(nums[i])
        }
        max
    }

    let n = nums.len();
	cmp::max(
		helper(nums.clone(), 0, n - 1),
		helper(nums.clone(), 1, n)
	)
}

#[test]
fn example_1() {
    assert_eq!(rob_ii(vec![2, 3, 2]), 3)
}

#[test]
fn example_2() {
    assert_eq!(rob_ii(vec![1, 2, 3, 1]), 4)
}

#[test]
fn example_3() {
    assert_eq!(rob_ii(vec![0]), 0)
}