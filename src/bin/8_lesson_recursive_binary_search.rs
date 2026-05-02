#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

fn main() {
    let tree = Some(Box::new(Node {
        val: 5,
        left: Some(Box::new(Node {
            val: 3,
            left: Some(Box::new(Node {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                val: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            val: 8,
            left: None,
            right: Some(Box::new(Node {
                val: 9,
                left: None,
                right: None,
            })),
        })),
    }));

    let depth = max_depth(&tree);

    println!("{depth}");

    let count = count_nodes(&tree);

    println!("{count}");

    let value = sum_tree(&tree);
    println!("{value}");

    let max_value = max_value(&tree);
    println!("{:?}", max_value);

    let is_containing = contains_tree(&tree, 3);

    println!("{is_containing}");
}

pub fn binary_search_recursive(nums: &[i32], left: usize, right: usize, target: i32) -> i32 {
    if left >= right {
        return -1;
    }
    let mut left = left;
    let mut right = right;
    let mid = left + (right - left) / 2;
    if nums[mid] == target {
        return mid as i32;
    }
    if nums[mid] > target {
        right = mid
    }
    if nums[mid] < target {
        left = mid + 1
    }
    binary_search_recursive(nums, left, right, target)
}

pub fn max_depth(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);

            1 + left_depth.max(right_depth)
        }
    }
}

pub fn count_nodes(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_count = count_nodes(&node.left);
            let right_count = count_nodes(&node.right);

            1 + left_count + right_count
        }
    }
}

pub fn sum_tree(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_value = sum_tree(&node.left);
            let right_value = sum_tree(&node.right);

            node.val + left_value + right_value
        }
    }
}

pub fn max_value(root: &Option<Box<Node>>) -> Option<i32> {
    match root {
        None => None,
        Some(node) => {
            let max_left = max_value(&node.left);
            let max_right = max_value(&node.right);
            max_left.max(max_right).max(Some(node.val))
        }
    }
}

pub fn contains_tree(root: &Option<Box<Node>>, target: i32) -> bool{
    match root{
        None => false,
        Some(node)=> {
            if node.val == target{
                return true
            }
        
            contains_tree(&node.left, target) || contains_tree(&node.left, target)
        }
    }
}
