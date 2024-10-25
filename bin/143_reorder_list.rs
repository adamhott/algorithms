// LeetCode 143. Reorder List

// You are given the head of a singly linked-list.

// The positions of a linked list of length = 7 for example, can intially be represented as:

// [0, 1, 2, 3, 4, 5, 6]

// Reorder the nodes of the linked list to be in the following order:

// [0, 6, 1, 5, 2, 4, 3]

// Notice that in the general case for a list of length = n the nodes are reordered to be in the following order:

// [0, n-1, 1, n-2, 2, n-3, ...]

// You may not modify the values in the list's nodes, but instead you must reorder the nodes themselves.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut vec = vec![];
        let mut current = head.take();
        
        while let Some(mut node) = current {
            current = node.next.take();
            vec.push(node);
        }

        let mut head_ref = head;
        let mut left = 0;
        let mut right = vec.len().wrapping_sub(1);

        while left <= right {
            head_ref.replace(vec[left].clone());
            head_ref = &mut head_ref.as_mut().unwrap().next;
            left += 1;

            if left <= right {
                head_ref.replace(vec[right].clone());
                head_ref = &mut head_ref.as_mut().unwrap().next;
                right = right.wrapping_sub(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = ListNode::new(val);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_reorder_list_example1() {
        let mut input = to_list(vec![1,2,3,4]);
        let expected = to_list(vec![1,4,2,3]);
        Solution::reorder_list(&mut input);
        assert_eq!(list_to_vec(input), list_to_vec(expected));
    }

    #[test]
    fn test_reorder_list_example2() {
        let mut input = to_list(vec![1,2,3,4,5]);
        let expected = to_list(vec![1,5,2,4,3]);
        Solution::reorder_list(&mut input);
        assert_eq!(list_to_vec(input), list_to_vec(expected));
    }
}
