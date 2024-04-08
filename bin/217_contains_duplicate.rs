// Leetcode 217. Contains Duplicate
// Given an integer array nums, return 
// true if any value appears at least twice 
// in the array, and return false if every 
// element is distinct.

use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset = HashSet::new();

    for n in nums {
        if hashset.contains(&n) {
            return true;
        }
        hashset.insert(n);
    }
    false
}

fn main() {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate_example_1() {
        let nums = [1, 2, 3, 1];
        assert!(contains_duplicate((&nums).to_vec()));
    }

    #[test]
    fn test_contains_duplicate_example_2() {
        let nums = [1, 2, 3, 4];
        assert!(!contains_duplicate((&nums).to_vec()));
    }

    #[test]
    fn test_contains_duplicate_example_3() {
        let nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(contains_duplicate((&nums).to_vec()));
    }
}
