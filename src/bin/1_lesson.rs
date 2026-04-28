use std::collections::HashMap;

fn main(){
	println!("{:?}{:?}{:?}", two_sum_hashmap(vec![2, 7, 11, 15], 9), // [0, 1]
two_sum_hashmap(vec![3, 2, 4], 6)  ,    // [1, 2]
two_sum_hashmap(vec![3, 3], 6)  )
}

pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut res: Vec<i32> = vec![];
	for i in 0..nums.len(){
		for j in 0..nums.len(){
			if nums[i] + nums[j] == target && i != j{
				res.insert(0, i as i32);
				res.insert(0, j as i32);
			}
		}
	}
	res
}

pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {

	let mut seen: HashMap<i32, usize> = HashMap::new();

	for i in 0..nums.len(){
		let current = nums[i];
		let complement = target - current;

		if let Some(&j) = seen.get(&complement){
			return vec![j as i32, i as i32]
		}

		seen.insert(current, i);
	}

	vec![]
}