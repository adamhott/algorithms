// LeetCode 124. Binary Tree Maximum Path Sum

// A path in a binary tree is a sequence of nodes where each pair
// of adjacent nodes in the sequence has an edge connecting them. 
// A node can only appear in the sequence at most once. Note that 
// the path does not need to pass through the root.

// The path sum of a path is the sum of the node's values in the path.

// Given the root of a binary tree, return the maximum path sum of any non-empty path.

// Definition for a binary tree node.
use std::cell::RefCell;
use std::cmp;
use std::collections::VecDeque;
use std::rc::Rc;

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = std::i32::MIN;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(n) = node {
                let left_max = cmp::max(dfs(n.borrow().left.clone(), res), 0);
                let right_max = cmp::max(dfs(n.borrow().right.clone(), res), 0);

                // Calculate the maximum path sum with the current node as the root
                *res = cmp::max(*res, n.borrow().val + left_max + right_max);

                // Return the maximum gain if we continue the path from the current node
                return n.borrow().val + cmp::max(left_max, right_max);
            }
            0
        }

        dfs(root, &mut res);
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

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root = build_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::max_path_sum(root), 6);
    }

    #[test]
    fn test_example_2() {
        let root = build_tree(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_path_sum(root), 42);
    }

    #[test]
    fn test_single_node() {
        let root = build_tree(vec![Some(-3)]);
        assert_eq!(Solution::max_path_sum(root), -3);
    }

    #[test]
    fn test_negative_values() {
        let root = build_tree(vec![
            Some(-10),
            Some(-20),
            Some(-30),
            Some(-5),
            Some(-15),
            None,
            None,
        ]);
        assert_eq!(Solution::max_path_sum(root), -5);
    }

    #[test]
    fn test_mixed_values() {
        let root = build_tree(vec![
            Some(2),
            Some(-1),
            Some(3),
            Some(-4),
            Some(-5),
            Some(4),
            Some(5),
        ]);
        assert_eq!(Solution::max_path_sum(root), 12);
    }    
    
}
