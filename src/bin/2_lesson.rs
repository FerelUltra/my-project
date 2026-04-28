use std::collections::{HashMap, HashSet};

fn main(){}


pub fn frequency_map(nums: Vec<i32>) -> HashMap<i32, i32> {
	let mut map: HashMap<i32, i32> = HashMap::new();

	for num in nums {
		let count = map.entry(num).or_insert(0);
		*count += 1;
	}

	map
}

pub fn first_duplicate(nums: Vec<i32>) -> i32{

	let mut set = HashSet::new();

	for num in nums{
		if !set.insert(num){
			return num
		}
	}

	-1
}