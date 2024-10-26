// LeetCode 19. Remove Nodoe From End of Linked List

// Given the head of a linked list, remove the nth node 
// from the end of the list and return its head.

// Solution from RustGym

type ListLink = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: ListLink,
}

impl ListNode {
    fn new(val: i32) -> ListLink {
        Some(Box::new(ListNode { val, next: None }))
    }
}

struct Solution;

impl Solution {
    fn remove_nth_from_end(mut head: ListLink, n: i32) -> ListLink {
        let mut v: Vec<ListLink> = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            v.push(Some(node));
        }
        let mut res = None;
        for (i, link) in v.into_iter().rev().enumerate() {
            if i != (n - 1) as usize {
                let mut node = link.unwrap();
                node.next = res;
                res = Some(node);
            }
        }
        res
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(vec: Vec<i32>) -> ListLink {
        let mut current = None;
        for &value in vec.iter().rev() {
            let mut node = ListNode::new(value);
            node.as_mut().unwrap().next = current;
            current = node;
        }
        current
    }

    fn list_to_vec(head: ListLink) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_case_1() {
        let list = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = Solution::remove_nth_from_end(list, 2);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_case_2() {
        let list = vec_to_list(vec![1]);
        let result = Solution::remove_nth_from_end(list, 1);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_case_3() {
        let list = vec_to_list(vec![1, 2]);
        let result = Solution::remove_nth_from_end(list, 1);
        assert_eq!(list_to_vec(result), vec![1]);
    }
}
