//! # 1553. Minimum number of days to eat n oranges
//! 
//! There are n oranges in the kitchen and you decide to eat some of
//! these oranges every day as follows:
//! *Eat one orange
//! *If the number of remaining oranges is divisible by 2 then you can eat
//! n/2 oranges
//! *If the number of remaining oranges is divisible by 3 then you can eat
//! 2*(n/3) oranges
//! 
//! You can only choose one of the actions per day.
//! 
//! Return the minimum unmber of days required to eat n oranges

use std::collections::HashMap;

pub fn min_days(n: i32) -> i32 {
	if n < 3 { return n };
	let mut mem: HashMap<i32, i32> = HashMap::new();

	fn helper(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
		if n <= 1 { return 1 }
		if let Some(days) = mem.get(&n) { return *days }
		let min = 1 + (n % 2 + helper(n / 2, mem)).min(n % 3 + helper(n / 3, mem));
		mem.insert(n, min);
		min
	}	

	helper(n, &mut mem)
}

