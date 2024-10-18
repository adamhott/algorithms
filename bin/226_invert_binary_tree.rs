// LeetCode 226. Invert a Binary Tree
// You are given the root of a binary tree root. 
// Invert the binary tree and return its root.

// Definition for a binary tree node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
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
    pub fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut node) = root {
            // Swap the left and right children
            let temp = node.left.take();
            node.left = Self::invert_tree(node.right.take());
            node.right = Self::invert_tree(temp);
            Some(node)
        } else {
            None
        }
    }
}

// Helper functions to create and test the tree structure
fn build_tree(values: &[Option<i32>]) -> Option<Box<TreeNode>> {
    use std::collections::VecDeque;

    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Some(Box::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().as_ref() as *const TreeNode as *mut TreeNode);

    let mut i = 1;
    while i < values.len() {
        if let Some(node) = queue.pop_front() {
            unsafe {
                if i < values.len() {
                    if let Some(val) = values[i] {
                        let left_node = Box::new(TreeNode::new(val));
                        (*node).left = Some(left_node);
                        queue.push_back((*node).left.as_mut().unwrap().as_mut() as *mut TreeNode);
                    }
                    i += 1;
                }

                if i < values.len() {
                    if let Some(val) = values[i] {
                        let right_node = Box::new(TreeNode::new(val));
                        (*node).right = Some(right_node);
                        queue.push_back((*node).right.as_mut().unwrap().as_mut() as *mut TreeNode);
                    }
                    i += 1;
                }
            }
        }
    }

    root
}

fn tree_to_vec(root: Option<Box<TreeNode>>) -> Vec<Option<i32>> {
    use std::collections::VecDeque;

    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        if let Some(n) = node {
            result.push(Some(n.val));
            queue.push_back(n.left);
            queue.push_back(n.right);
        } else {
            result.push(None);
        }
    }

    // Trim trailing `None` values
    while result.last() == Some(&None) {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(
            tree_to_vec(inverted),
            vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4)]
        );
    }

    #[test]
    fn example_2() {
        let root = build_tree(&[Some(3), Some(2), Some(1)]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted), vec![Some(3), Some(1), Some(2)]);
    }

    #[test]
    fn example_3() {
        let root = build_tree(&[]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted), vec![]);
    }
}
