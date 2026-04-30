fn main() {}

pub fn binary_search_recursive(nums: &[i32], left: usize, right: usize, target: i32) -> i32 {
	if left >= right{
		return -1
	}
	let mut left = left;
	let mut right = right;
	let mid = left + ( right - left) /2;
	if nums[mid] == target{
		return mid as i32;
	}
	if nums[mid] > target{
		right = mid
	}
	if nums[mid] < target {
		left = mid + 1
	}
	binary_search_recursive(nums, left, right, target)

}
