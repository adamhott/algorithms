// LeedCode 206. Reverse Linked List

// Given the head of a singly linked list, 
// reverse the list, and return the reversed list.

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

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array_to_list(array: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &value in array.iter().rev() {
            let mut new_node = ListNode::new(value);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }

    fn list_to_array(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut array = vec![];
        let mut current = list;
        while let Some(node) = current {
            array.push(node.val);
            current = node.next;
        }
        array
    }

    #[test]
    fn test_example_1() {
        let head = array_to_list(vec![1, 2, 3, 4, 5]);
        let reversed_list = Solution::reverse_list(head);
        let result_array = list_to_array(reversed_list);
        assert_eq!(result_array, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_example_2() {
        let head = array_to_list(vec![1, 2]);
        let reversed_list = Solution::reverse_list(head);
        let result_array = list_to_array(reversed_list);
        assert_eq!(result_array, vec![2, 1]);
    }

    #[test]
    fn test_example_3() {
        let head = array_to_list(vec![]);
        let reversed_list = Solution::reverse_list(head);
        let result_array = list_to_array(reversed_list);
        assert_eq!(result_array, vec![]);
    }
}
