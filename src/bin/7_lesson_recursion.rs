fn main(){
println!("{}",factorial_acc(5, 1));
}

pub fn sum_recursive(nums: &[i32], index: usize) -> i32 {
	if index == nums.len() {
		return 0;
	}

	nums[index] + sum_recursive(nums, index + 1)
} 

pub fn count_recursive(nums: &[i32], index: usize) -> i32{
	if index == nums.len(){
		return 0;
	}
	

	return 1 + count_recursive(nums, index + 1)
}

pub fn max_recursive(nums: &[i32], index: usize) -> i32{

	if index == nums.len() - 1{
		return nums[index]
	}

	let current = nums[index]; 
	let max_result = max_recursive(nums, index + 1);

	if max_result > current {
		max_result
	} else {
		current
	}
}

pub fn contains_recursive(nums: &[i32], index: usize, target: i32) -> bool{

	if index == nums.len(){
		return false
	}

	if nums[index] == target{
		return true
	}

	return contains_recursive(nums, index + 1, target)

}

pub fn find_index_recursive(nums: &[i32], index: usize, target: i32) -> i32{

	if index == nums.len(){
		return -1
	}

	if nums[index] == target{
		return index as i32
	}

	find_index_recursive(nums, index + 1, target)
}

pub fn print_reverse_recursive(nums: &[i32], index: usize){
	let len = nums.len();
	
	if index == len{
		return
	}

	println!("{}",nums[len - 1 - index]);
	print_reverse_recursive(nums, index + 1);
}

pub fn print_reverse_recursive_postorder(nums: &[i32], index: usize){
	if index == nums.len(){
		return
	}

	print_reverse_recursive_postorder(nums, index + 1);
	println!("{}", nums[index])
}

pub fn reverse_recursive(nums: &[i32], index: usize) -> Vec<i32> {

	if index == nums.len(){
		return Vec::new();
	}

	let mut result = reverse_recursive(nums, index + 1);

	result.push(nums[index]);

	result
}

pub fn factorial(n: u64) -> u64{
	if n <= 1{
		return 1;
	}

	factorial(n - 1) * n
}

pub fn fibonacci(n: u64) -> u64{
	if n == 0{
		return 0;
	}
	if n == 1 {
		return 1;
	}

	fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn factorial_acc(n: u64, acc: u64) -> u64{

	if n <= 1{
		return acc;
	}
	
	factorial_acc(n-1, acc*n)
}

pub fn sum_acc(nums: &[i32], index: usize, acc: i32) -> i32{

	if index == nums.len(){
		return acc
	}

	sum_acc(nums, index + 1, acc + nums[index])
}