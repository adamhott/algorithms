// Leetcode 347. Top K Frequent Elements
// Given an integer array nums and an 
// integer k, return the k most frequent 
// elements. You may return the answer in any order.

use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count: HashMap<i32, usize> = HashMap::new();
    let mut freq: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];

    // Count the frequency of each element
    for &num in &nums {
        *count.entry(num).or_insert(0) += 1;
    }

    // Group numbers by their frequency
    for (num, frequency) in count {
        freq[frequency].push(num);
    }

    // Collect the top k frequent elements
    let mut res = Vec::new();
    for i in (1..freq.len()).rev() {
        for &num in &freq[i] {
            res.push(num);
            if res.len() == k as usize {
                return res;
            }
        }
    }

    res
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_example2() {
        let nums = vec![1];
        let k = 1;
        let result = top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }
}

