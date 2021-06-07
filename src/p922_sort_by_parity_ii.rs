pub fn sort_array_by_parity_i(nums: Vec<i32>) -> Vec<i32> {
	let n = nums.len();
	let mut even: Vec<i32> = Vec::with_capacity(n/2);
	let mut odd: Vec<i32> = Vec::with_capacity(n/2);
	for num in nums.into_iter() {
		if num % 2 == 0 { even.push(num) } else { odd.push(num) }
	}
	(0..n).into_iter()
		.map(|x| if x % 2 == 0 { even[x/2] } else { odd[x/2] })
		.collect()
}