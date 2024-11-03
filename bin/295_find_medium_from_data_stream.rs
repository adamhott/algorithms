// LeetCode 295. Find Medium from Data Stream

// The median is the middle value in an ordered integer list. 
// If the size of the list is even, there is no middle value, 
// and the median is the mean of the two middle values.

// For example, for arr = [2,3,4], the median is 3.
// For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.

// Implement the MedianFinder class:

// MedianFinder() initializes the MedianFinder object.

// void addNum(int num) adds the integer num from the data stream 
// to the data structure.

// double findMedian() returns the median of all elements so far. 
// Answers within 10-5 of the actual answer will be accepted.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    small: BinaryHeap<i32>,            // Max heap for the smaller half
    large: BinaryHeap<Reverse<i32>>,   // Min heap for the larger half
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if let Some(&Reverse(top)) = self.large.peek() {
            if num > top {
                self.large.push(Reverse(num));
            } else {
                self.small.push(num);
            }
        } else {
            self.small.push(num);
        }

        // Balance the heaps
        if self.small.len() > self.large.len() + 1 {
            if let Some(val) = self.small.pop() {
                self.large.push(Reverse(val));
            }
        }
        if self.large.len() > self.small.len() + 1 {
            if let Some(Reverse(val)) = self.large.pop() {
                self.small.push(val);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else if self.large.len() > self.small.len() {
            self.large.peek().unwrap().0 as f64
        } else {
            (*self.small.peek().unwrap() as f64 + self.large.peek().unwrap().0 as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut median_finder = MedianFinder::new();
        
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert_eq!(median_finder.find_median(), 1.5);
        
        median_finder.add_num(3);
        assert_eq!(median_finder.find_median(), 2.0);
    }
}
