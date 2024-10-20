// LeetCode 100. Same Tree

// Given the roots of two binary trees p and q, 
// write a function to check if they are the same or not.

// Two binary trees are considered the same if they are 
// structurally identical, and the nodes have the same value.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(PartialEq, Eq, Debug, Clone)]
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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p_node), Some(q_node)) => {
            let p = p_node.borrow();
            let q = q_node.borrow();
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn example_1() {
        let p = build_tree(vec![Some(1), Some(2), Some(3)]);
        let q = build_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(is_same_tree(p, q), true);
    }

    #[test]
    fn example_2() {
        let p = build_tree(vec![Some(1), Some(2), None]);
        let q = build_tree(vec![Some(1), None, Some(2)]);
        assert_eq!(is_same_tree(p, q), false);
    }

    #[test]
    fn example_3() {
        let p = build_tree(vec![Some(1), Some(2), Some(1)]);
        let q = build_tree(vec![Some(1), Some(1), Some(2)]);
        assert_eq!(is_same_tree(p, q), false);
    }
}
