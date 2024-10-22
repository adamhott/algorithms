// LeetCode 230. Kth Smallest Element in a BST

// Given the root of a binary search tree, and an integer k, 
// return the kth smallest value (1-indexed) of all the values 
// of the nodes in the tree.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut curr = root;
        let mut k = k;

        while stack.len() > 0 || curr.is_some() {
            while let Some(node) = curr {
                stack.push(Rc::clone(&node));
                curr = node.borrow().left.clone();
            }
            curr = stack.pop();
            if let Some(node) = curr {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                curr = node.borrow().right.clone();
            }
        }

        -1 // This should not be reached if k is valid
    }
}

// Helper function to create a binary tree from a vector of Option values.
fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut index = 1;
    while index < values.len() {
        let current = queue.pop_front().unwrap();

        if let Some(val) = values[index] {
            let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
            current.borrow_mut().left = Some(Rc::clone(&left_node));
            queue.push_back(left_node);
        }
        index += 1;

        if index < values.len() {
            if let Some(val) = values[index] {
                let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().right = Some(Rc::clone(&right_node));
                queue.push_back(right_node);
            }
            index += 1;
        }
    }

    Some(root)
}

fn main() {
    todo!();
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root = build_tree(vec![
            Some(3), Some(1), Some(4), None, Some(2)
        ]);
        assert_eq!(Solution::kth_smallest(root, 1), 1);
    }

    #[test]
    fn test_example_2() {
        let root = build_tree(vec![
            Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)
        ]);
        assert_eq!(Solution::kth_smallest(root, 3), 3);
    }
}
