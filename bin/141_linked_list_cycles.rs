// LeetCode 141. Linked List Cycles

// Given head, the head of a linked list, determine 
// if the linked list has a cycle in it.

// There is a cycle in a linked list if there is some node 
// in the list that can be reached again by continuously
// following the next pointer. Internally, pos is used to denote 
// the index of the node that tail's next pointer is connected to. 
// Note that pos is not passed as a parameter.

// Return true if there is a cycle in the linked list. Otherwise, return false.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head;

        while let Some(fast_node) = fast {
            if fast_node.borrow().next.is_none() {
                return false;
            }
            slow = slow.unwrap().borrow().next.clone();
            fast = fast_node.borrow().next.as_ref().unwrap().borrow().next.clone();

            if Rc::ptr_eq(&slow.as_ref().unwrap(), &fast.as_ref().unwrap()) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle_1() {
        let node1 = Rc::new(RefCell::new(ListNode::new(3)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(0)));
        let node4 = Rc::new(RefCell::new(ListNode::new(-4)));

        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node4.clone());
        node4.borrow_mut().next = Some(node2.clone()); // Creates a cycle

        assert!(Solution::has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_2() {
        let node1 = Rc::new(RefCell::new(ListNode::new(1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));

        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node1.clone()); // Creates a cycle

        assert!(Solution::has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_3() {
        let node1 = Rc::new(RefCell::new(ListNode::new(1)));

        assert!(!Solution::has_cycle(Some(node1)));
    }
}

