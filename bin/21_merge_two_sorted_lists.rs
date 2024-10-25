// LeetCode 21. Merge Two Sorted Lists

// You are given the heads of two sorted linked lists list1 and list2.

// Merge the two lists into one sorted list. The list should be made by 
// splicing together the nodes of the first two lists.

// Return the head of the merged linked list.

// LeetCode 21. Merge Two Sorted Lists

// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted list. The list should be made by 
// splicing together the nodes of the first two lists.
// Return the head of the merged linked list.

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        while let (Some(ref mut l1), Some(ref mut l2)) = (list1.as_mut(), list2.as_mut()) {
            if l1.val < l2.val {
                let next = l1.next.take();
                current.next = list1.take();
                list1 = next;
            } else {
                let next = l2.next.take();
                current.next = list2.take();
                list2 = next;
            }
            current = current.next.as_mut().unwrap();
        }

        current.next = if list1.is_some() { list1 } else { list2 };

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let new_node = ListNode::new(val);
            let mut boxed_node = Box::new(new_node);
            boxed_node.next = current;
            current = Some(boxed_node);
        }
        current
    }

    fn vec_from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_merge_two_lists_example_1() {
        let list1 = list_from_vec(vec![1, 2, 4]);
        let list2 = list_from_vec(vec![1, 3, 4]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(vec_from_list(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_example_2() {
        let list1 = list_from_vec(vec![]);
        let list2 = list_from_vec(vec![]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(vec_from_list(result), vec![]);
    }

    #[test]
    fn test_merge_two_lists_example_3() {
        let list1 = list_from_vec(vec![]);
        let list2 = list_from_vec(vec![0]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(vec_from_list(result), vec![0]);
    }
}

fn main() {
    todo!();
}
