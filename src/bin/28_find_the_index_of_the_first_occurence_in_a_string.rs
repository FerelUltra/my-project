use std::fmt::Debug;

pub fn str_str(haystack: String, needle: String) -> i32 {
	match haystack.find(&needle) {
        Some(i) => i as i32,
        None => -1,
    }
}

pub fn str_str_without_find(haystack: String, needle: String) -> i32{
	let h = haystack.as_bytes();
	let n = needle.as_bytes();

	if n.is_empty(){
		return 0
	}

	if n.len() > h.len() {
		return -1
	}

	for start in 0..= h.len() - n.len() {
		let mut matched = true;

		for j in 0..n.len() {
			if h[start + j] != n[j] {
				matched = false;
				break;
			}
		}   
		if matched {
			return start as i32
		}
	}

	-1
}

fn main() {
	println!("{:?}",str_str("leetcode".to_string(), "leeto".to_string()))
}
