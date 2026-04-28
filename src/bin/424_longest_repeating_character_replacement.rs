use std::{cmp, collections::HashMap};

fn main() {}

pub fn character_replacement(s: String, k: i32) -> i32 {
	let mut map: HashMap<char, i32> = HashMap::new();
	let mut max = 0;
	let mut left = 0;
	let mut max_freq = 0;

	let chars: Vec<char> = s.chars().collect();

	for right in 0..chars.len(){
		let count = map.entry(chars[right]).or_insert(0);
		*count += 1;

		max_freq = cmp::max(max_freq, *count);
		
		let window_len = (right - left + 1) as i32;

		if window_len - max_freq > k{
			let left_count = map.get_mut(&chars[left]).unwrap();
			*left_count -= 1;
			left += 1;
		}
		max = cmp::max(max, window_len)
	}
	max
}
