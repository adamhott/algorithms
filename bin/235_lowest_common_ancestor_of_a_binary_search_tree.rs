// LeetCode 235. Lowest Common Ancestor of a Binary Search Tree

// Given a binary search tree (BST), find the lowest common ancestor (LCA) 
// node of two given nodes in the BST.

// According to the definition of LCA on Wikipedia: 
// “The lowest common ancestor is defined between two nodes p and q 
// as the lowest node in T that has both p and q as descendants 
// (where we allow a node to be a descendant of itself).”

// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Rc<RefCell<TreeNode>>,
        q: Rc<RefCell<TreeNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;

        while let Some(node) = current {
            let node_val = node.borrow().val;
            let p_val = p.borrow().val;
            let q_val = q.borrow().val;

            if node_val < p_val && node_val < q_val {
                current = node.borrow().right.clone();
            } else if node_val > p_val && node_val > q_val {
                current = node.borrow().left.clone();
            } else {
                return Some(node);
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

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
    fn test_example_1() {
        let root = build_tree(vec![
            Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5),
        ]);
        let p = Rc::new(RefCell::new(TreeNode::new(2)));
        let q = Rc::new(RefCell::new(TreeNode::new(8)));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 6);
    }

    #[test]
    fn test_example_2() {
        let root = build_tree(vec![
            Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5),
        ]);
        let p = Rc::new(RefCell::new(TreeNode::new(2)));
        let q = Rc::new(RefCell::new(TreeNode::new(4)));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_example_3() {
        let root = build_tree(vec![Some(2), Some(1)]);
        let p = Rc::new(RefCell::new(TreeNode::new(2)));
        let q = Rc::new(RefCell::new(TreeNode::new(1)));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}

