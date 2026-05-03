use std::{mem::swap};
use std::collections::VecDeque;
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

    let mut result: Vec<i32> = Vec::new();
    let preorder_result = preorder(&tree, result.as_mut());

    println!("{:?}", result);

    let mut inorder_result: Vec<i32> = Vec::new();

    inorder(&tree, &mut inorder_result.as_mut());
    println!("{:?}", inorder_result);

    let mut postorder_result: Vec<i32> = Vec::new();

    postorder(&tree, &mut postorder_result.as_mut());
    println!("{:?}", postorder_result);
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

pub fn contains_tree(root: &Option<Box<Node>>, target: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            if node.val == target {
                return true;
            }

            contains_tree(&node.left, target) || contains_tree(&node.left, target)
        }
    }
}

pub fn preorder(root: &Option<Box<Node>>, result: &mut Vec<i32>) {
    match root {
        None => return,
        Some(node) => {
            result.push(node.val);
            preorder(&node.left, result);
            preorder(&node.right, result);
        }
    }
}

pub fn inorder(root: &Option<Box<Node>>, result: &mut Vec<i32>) {
    match root {
        None => return,
        Some(node) => {
            inorder(&node.left, result);
            result.push(node.val);
            inorder(&node.right, result);
        }
    }
}

pub fn postorder(root: &Option<Box<Node>>, result: &mut Vec<i32>) {
    match root {
        None => return,
        Some(node) => {
            postorder(&node.left, result);
            postorder(&node.right, result);
            result.push(node.val);
        }
    }
}

pub fn invert_tree(root: &mut Option<Box<Node>>) {
    match root {
        None => return,
        Some(node) => {
            swap(&mut node.left, &mut node.right);

            invert_tree(&mut node.left);
            invert_tree(&mut node.right);
        }
    }
}

pub fn same_tree(root1: &Option<Box<Node>>, root2: &Option<Box<Node>>) -> bool {
    match (root1, root2) {
        (None, None) => true,
        (Some(node1), Some(node2)) => {
            node1.val == node2.val
                && same_tree(&node1.left, &node2.left)
                && same_tree(&node1.right, &node2.right)
        }
        _ => false,
    }
}

pub fn is_mirror(left: &Option<Box<Node>>, right: &Option<Box<Node>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(node_left), Some(node_right)) =>{
            node_left.val == node_right.val 
    && is_mirror(&node_left.left, &node_right.right)
    && is_mirror(&node_left.right, &node_right.left)
        }
        _ => false,
    }
}

pub fn symmetric(root: &Option<Box<Node>>) -> bool {
    match root{
        None=> true,
        Some(node) => is_mirror(&node.left, &node.right)
    }
}

    pub fn has_path_sum(root: &Option<Box<Node>>, target: i32) -> bool{
        match root{
            None => false,
            Some(node) =>{
    
                let is_leaf = node.left.is_none() && node.right.is_none();
                
                if is_leaf{
                    return node.val == target;
                }
                let new_target = target - node.val;
    
                has_path_sum(&node.left, new_target) || has_path_sum(&node.right, new_target)
            }
        }
    }  

    pub fn is_valid_bst(root: &Option<Box<Node>>) -> bool{
        validate(root, None, None)
    }

    pub fn validate(root: &Option<Box<Node>>, min: Option<i32>, max: Option<i32>) -> bool{
        match root{
            None => true,
            Some(node) => {

                if let Some(min_value) = min {
                    if node.val <=min_value {
                        return false
                    }
                }

                if let Some(max_value) = max{
                    if node.val >= max_value{
                        return false
                    }
                }
            

                validate(&node.left, min, Some(node.val)) && validate(&node.right,  Some(node.val), max)
            }
        }
    }

    pub fn bfs_values(root: &Option<Box<Node>>) -> Vec<i32> {
        let mut result  = Vec::new();
        let mut queue: VecDeque<&Node> = VecDeque::new();

        if let Some(node) = root{
            queue.push_back(node.as_ref());
        }

        while let Some(node) = queue.pop_front(){
                   result.push(node.val);
                   
            if let Some(left_node) = &node.left{
                queue.push_back(left_node.as_ref());
            }
            if let Some(right_node) = &node.right{
                queue.push_back(right_node.as_ref());
            }
        }

        result
    }

    pub fn level_order(root: &Option<Box<Node>>) -> Vec<Vec<i32>>{
        
    }