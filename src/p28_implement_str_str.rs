//! # 28. Implement str_str()
//! 
//! Return the index of the first occurance of `needle` in `haystack`,
//! or `-1` if `needle` is not part of `haystack`
//! 
//! If `needle` is an empty string, return 0

pub fn str_str(haystack: String, needle: String) -> i32 {
	// This is an edge case which is application-specific,
	// but in this case the prompt specifies how to handle it
	if needle.len() == 0 { return 0 }
	
	for i in 0..haystack.len() {
		if haystack.split_at(i).1.starts_with(&needle) {
			return i as i32
		}
	}
	-1
}

#[test]
fn example_1() {
	assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2)
}

#[test]
fn example_2() {
	assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1)
}

#[test]
fn example_3() {
	assert_eq!(str_str("".to_string(), "".to_string()), 0)
}