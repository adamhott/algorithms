// LeetCode 297. Serialize and Deserialize Binary Tree

// Serialization is the process of converting a data structure 
// or object into a sequence of bits so that it can be stored 
// in a file or memory buffer, or transmitted across a network 
// connection link to be reconstructed later in the same or another 
// computer environment.

// Design an algorithm to serialize and deserialize a binary tree. 
// There is no restriction on how your serialization/deserialization 
// algorithm should work. You just need to ensure that a binary tree 
// can be serialized to a string and this string can be deserialized 
// to the original tree structure.

// Clarification: The input/output format is the same as how LeetCode 
// serializes a binary tree. You do not necessarily need to follow this 
// format, so please be creative and come up with different approaches yourself.

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

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

pub struct Codec;

impl Codec {
    // Encodes a tree to a single string.
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = Vec::new();

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<String>) {
            if let Some(node) = node {
                let node = node.borrow();
                res.push(node.val.to_string());
                dfs(node.left.clone(), res);
                dfs(node.right.clone(), res);
            } else {
                res.push("N".to_string());
            }
        }

        dfs(root, &mut res);
        res.join(",")
    }

    // Decodes your encoded data to tree.
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<&str> = data.split(',').collect();
        let mut index = 0;

        fn dfs(vals: &Vec<&str>, index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            if vals[*index] == "N" {
                *index += 1;
                return None;
            }
            let val = vals[*index].parse::<i32>().unwrap();
            *index += 1;
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = dfs(vals, index);
            node.borrow_mut().right = dfs(vals, index);
            Some(node)
        }

        dfs(&vals, &mut index)
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let codec = Codec;
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]);
        let serialized = codec.serialize(root.clone());
        assert_eq!(serialized, "1,2,N,N,3,4,N,N,5,N,N");

        let deserialized = codec.deserialize(serialized);
        assert_eq!(root, deserialized);
    }

    #[test]
    fn test_example_2() {
        let codec = Codec;
        let root = build_tree(vec![]);
        let serialized = codec.serialize(root.clone());
        assert_eq!(serialized, "N");

        let deserialized = codec.deserialize(serialized);
        assert_eq!(root, deserialized);
    }

    #[test]
    fn test_empty_tree() {
        let codec = Codec;
        let root = None;
        let serialized = codec.serialize(root.clone());
        assert_eq!(serialized, "N");

        let deserialized = codec.deserialize(serialized);
        assert_eq!(root, deserialized);
    }

    #[test]
    fn test_single_node_tree() {
        let codec = Codec;
        let root = build_tree(vec![Some(1)]);
        let serialized = codec.serialize(root.clone());
        assert_eq!(serialized, "1,N,N");

        let deserialized = codec.deserialize(serialized);
        assert_eq!(root, deserialized);
    }
}
