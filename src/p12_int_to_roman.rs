//! # 12. Integer to Roman
//! Convert a given positive integer between 1 and 3999 into roman numerals

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