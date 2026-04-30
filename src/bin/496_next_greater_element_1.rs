use std::collections::HashMap;

fn main() {}

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for num in nums2 {
        while let Some(&last_num) = stack.last() {
            if num > last_num {
				stack.pop();
				map.insert(last_num, num);
			} else {
				break;
			}
            
			
        }

		stack.push(num);
    }

	for num in nums1{
		result.push(*map.get(&num).unwrap_or(&-1));
	}

    result
}


fn i_remember_i_swear(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>{
	let mut stack: Vec<i32> = Vec::new();
	let mut map: HashMap<i32, i32> = HashMap::new();
	let mut result: Vec<i32> = Vec::new();

	for num in nums2{
		while let Some(&last_num) = stack.last(){
			if num > last_num{
				map.insert(last_num, num);
				stack.pop();
			} else {
				break;
			}
		}
		stack.push(num);
	}

	for num in nums1{
		result.push(*map.get(&num).unwrap_or(&-1))
	}

	result

}