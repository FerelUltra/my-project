pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    
	let mut slow = 1;

	for fast in 1..nums.len(){
		if nums[slow - 1] != nums[fast] {
			nums[slow] = nums[fast];
			slow += 1;
		}
	}
	slow as i32
}

fn main() {}
