fn main() {
    let example = &mut vec![1, 0, 1, 0, 2, 0, 3, 3];
    println!("{:?}", move_zeroes(example))
}

// [0, 1, 0, 2, 0, 3, 3]
pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut slow = 0;

    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    for i in slow..nums.len() {
        nums[i] = 0;
    }
    nums
}
