// LeetCode 105. Construct Binary Tree from Preorder and Inorder Traversal

// Given two integer arrays preorder and inorder where preorder is the 
// preorder traversal of a binary tree and inorder is the inorder traversal 
// of the same tree, construct and return the binary tree.

// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let root_val = preorder[0];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        let mid = inorder.iter().position(|&x| x == root_val).unwrap();
        root.borrow_mut().left = Self::build_tree(preorder[1..mid + 1].to_vec(), inorder[..mid].to_vec());
        root.borrow_mut().right = Self::build_tree(preorder[mid + 1..].to_vec(), inorder[mid + 1..].to_vec());

        Some(root)
    }
}

// Helper function to create a binary tree from a vector of Option values.
fn build_tree_from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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

// Helper function to check if two binary trees are equal
fn are_trees_equal(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (t1, t2) {
        (Some(n1), Some(n2)) => {
            let n1 = n1.borrow();
            let n2 = n2.borrow();
            n1.val == n2.val
                && are_trees_equal(n1.left.clone(), n2.left.clone())
                && are_trees_equal(n1.right.clone(), n2.right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let expected = build_tree_from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::build_tree(preorder, inorder);
        assert!(are_trees_equal(result, expected));
    }

    #[test]
    fn test_example_2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let expected = build_tree_from_vec(vec![Some(-1)]);
        let result = Solution::build_tree(preorder, inorder);
        assert!(are_trees_equal(result, expected));
    }
}

fn main() {
    todo!();
}
