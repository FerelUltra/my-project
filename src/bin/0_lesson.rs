fn main(){
	
}

pub fn contains_with_contains_method(nums: Vec<i32>, target: i32) -> bool {
	nums.contains(&target)
}
pub fn contains(nums: Vec<i32>, target: i32) -> bool{
	for i in 0..nums.len(){
		if nums[i] == target{
			return true
		}
	}
	return false
}

pub fn find_max(nums: Vec<i32>) -> i32 {
	let mut max = nums[0];

	for i in 1..nums.len(){
		if max < nums[i]{
			max = nums[i]
		}
	}
	return max
}


pub fn count_target(nums: Vec<i32>, target: i32) -> i32 {
	let mut count = 0;

	for i in 0..nums.len(){
		if nums[i] == target{
			count += 1;
		}
	}

	count
}