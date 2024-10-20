// LeetCode 572. Subtree of Another Tree

// Given the roots of two binary trees root and subRoot, return true 
// if there is a subtree of root with the same structure and node values 
// of subRoot and false otherwise.

// A subtree of a binary tree tree is a tree that consists of a node in 
// tree and all of this node's descendants. The tree tree could also be 
// considered as a subtree of itself.


use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(PartialEq, Eq, Clone, Debug)]
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
    // Main function to check if subRoot is a subtree of root
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if sub_root.is_none() {
            return true; // An empty tree is always a subtree
        }
        if root.is_none() {
            return false; // If the main tree is empty, it can't contain any subtree
        }

        if Self::same_tree(&root, &sub_root) {
            return true;
        }

        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();

        Self::is_subtree(left, sub_root.clone()) || Self::is_subtree(right, sub_root)
    }

    // Helper function to determine if two trees are identical
    fn same_tree(
        s: &Option<Rc<RefCell<TreeNode>>>,
        t: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (s, t) {
            (Some(s_node), Some(t_node)) => {
                s_node.borrow().val == t_node.borrow().val
                    && Self::same_tree(&s_node.borrow().left, &t_node.borrow().left)
                    && Self::same_tree(&s_node.borrow().right, &t_node.borrow().right)
            }
            (None, None) => true,
            _ => false,
        }
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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = build_tree(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
        ]);
        let sub_root = build_tree(vec![Some(4), Some(1), Some(2)]);
        assert_eq!(Solution::is_subtree(root, sub_root), true);
    }

    #[test]
    fn example_2() {
        let root = build_tree(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root = build_tree(vec![Some(4), Some(1), Some(2)]);
        assert_eq!(Solution::is_subtree(root, sub_root), false);
    }
}
