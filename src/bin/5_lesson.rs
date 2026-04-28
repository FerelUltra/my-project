use std::{cmp, collections::HashSet};

fn main() {
    let example = String::from("abcabcbb");
    let example2 = String::from("abcabcbababababbdef");
    println!("{}", length_of_longest_substring(example));
    println!("{}", length_of_longest_substring(example2));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    let mut max = 0;

    let mut left = 0;

    for right in 0..chars.len() {

		while set.contains(&chars[right]) {
                set.remove(&chars[left]);
                left += 1;
            }

		set.insert(chars[right]);

        max = cmp::max(max, right - left + 1);
    }

    max as i32
}

