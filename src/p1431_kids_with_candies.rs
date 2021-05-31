//! # 1431. Kids with candies
//! Given the array 'candies' and the integer 'extra_candies', where
//! 'candies[i]' represents the number of candles that the ith kid has,
//! return a boolean array indicating if there is a way for each kind that
//! 'extra_candies' may be distributed so that they have the greatest number
//! of candies


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