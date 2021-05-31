//! # 121. Max profit
//! ## Given an array indicating the price of a stock on a particular day,
//! ## return the maximum profit you can acheive by buying on one day and
//! ## selling on a future day

use std::cmp;

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