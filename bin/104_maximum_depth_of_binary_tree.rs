use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                1 + std::cmp::max(
                    Solution::max_depth(node.left.clone()),
                    Solution::max_depth(node.right.clone()),
                )
            }
        }
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
    fn example1() {
        let root = build_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn example2() {
        let root = build_tree(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(root), 2);
    }
}
