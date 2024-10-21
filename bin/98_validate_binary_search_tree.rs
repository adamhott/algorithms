// LeetCode 98. Validate Binary Search Tree

// Given the root of a binary tree, determine if it is a valid binary search tree (BST).

//A valid BST is defined as follows:

// The left subtree of a node contains only nodes with keys less than the node's key.
// The right subtree of a node contains only nodes with keys greater than the node's key.
// Both the left and right subtrees must also be binary search trees.

use std::cell::RefCell;
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn valid(node: &Option<Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
            if let Some(n) = node {
                let n = n.borrow();
                if n.val as i64 <= left || n.val as i64 >= right {
                    return false;
                }
                return valid(&n.left, left, n.val as i64) && valid(&n.right, n.val as i64, right);
            }
            true
        }

        valid(&root, i64::MIN, i64::MAX)
    }
}

// Helper function to create a binary tree from a vector of Option values.
fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = build_tree(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::is_valid_bst(root), true);
    }

    #[test]
    fn example_2() {
        let root = build_tree(vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn example_3() {
        let root = build_tree(vec![Some(10), Some(5), Some(15), None, None, Some(6), Some(20)]);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn example_4() {
        let root = build_tree(vec![Some(1), Some(1)]);
        assert_eq!(Solution::is_valid_bst(root), false);
    }
}

fn main() {
    todo!();
}


