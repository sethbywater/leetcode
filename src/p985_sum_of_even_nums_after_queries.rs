pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
	let n = queries.len();
	queries.into_iter()
		.fold((Vec::with_capacity(n), nums.clone()), |mut acc, x| {
			acc.1[x[1] as usize] += x[0];
			acc.0.push(acc.1.iter().filter(|x| **x % 2 == 0).sum());
			acc 
		})
		.0
		
}