//LeetCode 33. Search in Rotated Sorted Array
//There is an integer array nums sorted in 
//ascending order (with distinct values).

//Prior to being passed to your function, nums is possibly 
//rotated at an unknown pivot index k (1 <= k < nums.length) 
//such that the resulting array is [nums[k], nums[k+1], ..., 
//nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). 

//For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 
//and become [4,5,6,7,0,1,2].

//Given the array nums after the possible rotation 
//and an integer target, return the index of target 
//if it is in nums, or -1 if it is not in nums.

//You must write an algorithm with O(log n) runtime complexity.

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let mid = (l + r) / 2;
            if target == nums[mid as usize] {
                return mid;
            }

            // left sorted portion
            if nums[l as usize] <= nums[mid as usize] {
                if target > nums[mid as usize] || target < nums[l as usize] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            // right sorted portion
            else {
                if target < nums[mid as usize] || target > nums[r as usize] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }
        }

        -1
    }
}


fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let expected = 4;
        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let expected = -1;
        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1];
        let target = 0;
        let expected = -1;
        assert_eq!(Solution::search(nums, target), expected);
    }
}
