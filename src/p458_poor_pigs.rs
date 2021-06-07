pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
	if buckets == 1 { return 0 }
	let tries = minutes_to_test / minutes_to_die + 1;
	let mut pigs = 1;
	while tries.pow(pigs as u32) < buckets { pigs += 1 }
	pigs
}