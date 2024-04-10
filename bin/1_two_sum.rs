// Leetcode 1. Two Sum
// Given an array of integers nums and an integer target, 
//return indices of the two numbers such that they add up to target.

//You may assume that each input would have exactly one solution, 
//and you may not use the same element twice.

//You can return the answer in any order.

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut prev_map = HashMap::new(); // val -> index

    for (i, &n) in nums.iter().enumerate() {
        let diff = target - n;
        if let Some(&index) = prev_map.get(&diff) {
            return vec![index as i32, i as i32];
        }
        prev_map.insert(n, i);
    }
    vec![]
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}

