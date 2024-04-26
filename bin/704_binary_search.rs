
//Given an array of integers nums which is sorted in 
//ascending order, and an integer target, write a 
//function to search target in nums. If target exists, 
//then return its index. Otherwise, return -1.

//You must write an algorithm with O(log n) runtime complexity.


struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() as isize - 1);

        while l <= r {
            let m = l + ((r - l) / 2); // Avoids overflow compared to (l + r) / 2
            if nums[m as usize] > target {
                r = m - 1;
            } else if nums[m as usize] < target {
                l = m + 1;
            } else {
                return m as i32; // Found the target, return the index
            }
        }

        -1 // Target not found, return -1
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let expected_output = 4; // The index of 9 in nums
        assert_eq!(Solution::search(nums, target), expected_output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let expected_output = -1; // 2 is not in the list
        assert_eq!(Solution::search(nums, target), expected_output);
    }
}