// LeetCode 102. Binary Tree Level Order Traversal

// Given the root of a binary tree, return the level order traversal 
// of its nodes' values. (i.e., from left to right, level by level).

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

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let mut level = Vec::new();
            let level_size = queue.len();

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node_ref = node.borrow();
                    level.push(node_ref.val);

                    if let Some(left) = &node_ref.left {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node_ref.right {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }
            res.push(level);
        }

        res
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

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root = build_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_example_2() {
        let root = build_tree(vec![Some(1)]);
        let expected = vec![vec![1]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_example_3() {
        let root = build_tree(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
    }
}

fn main() {
    todo!();
}
