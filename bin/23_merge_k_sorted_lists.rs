// LeetCode 23. Merge K Sorted Lists

// You are given an array of k linked-lists lists, 
// each linked-list is sorted in ascending order.

// Merge all the linked-lists into one sorted linked-list and return it.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut lists = lists;
        while lists.len() > 1 {
            let mut merged_lists = Vec::new();
            for i in (0..lists.len()).step_by(2) {
                let l1 = lists[i].take();
                let l2 = if i + 1 < lists.len() {
                    lists[i + 1].take()
                } else {
                    None
                };
                merged_lists.push(Solution::merge_two_lists(l1, l2));
            }
            lists = merged_lists;
        }
        lists.into_iter().next().unwrap()
    }

    fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let (mut l1, mut l2) = (l1, l2);
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                tail.next = l1;
                tail = tail.next.as_mut().unwrap();
                l1 = tail.next.take();
            } else {
                tail.next = l2;
                tail = tail.next.as_mut().unwrap();
                l2 = tail.next.take();
            }
        }

        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &value in values.iter().rev() {
            let new_node = Some(Box::new(ListNode {
                val: value,
                next: head,
            }));
            head = new_node;
        }
        head
    }

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            values.push(node.val);
            current = node.next;
        }
        values
    }

    #[test]
    fn test_merge_k_lists_example1() {
        let lists = vec![
            build_list(vec![1, 4, 5]),
            build_list(vec![1, 3, 4]),
            build_list(vec![2, 6]),
        ];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_merge_k_lists_example2() {
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_merge_k_lists_example3() {
        let lists = vec![build_list(vec![])];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), vec![]);
    }
}
