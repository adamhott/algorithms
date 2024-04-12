// Leetcode 128. Longest Consecutive Sequence
// Given an unsorted array of integers nums, 
// return the length of the longest consecutive 
// elements sequence.

// You must write an algorithm that runs in O(n) time.

use std::collections::HashSet;

struct Solution;

impl Solution {
    fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &n in &num_set {
            // Check if it's the start of a sequence
            if !num_set.contains(&(n - 1)) {
                let mut length = 1;
                while num_set.contains(&(n + length)) {
                    length += 1;
                }
                longest = longest.max(length);
            }
        }

        longest
    }
}

fn main() {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_consecutive_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn test_longest_consecutive_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }
}
