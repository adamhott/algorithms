// LeetCode 15 - 3Sum

//Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] 
//such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        
        nums.sort_unstable(); // Sorts the list to use the two-pointer technique

        for i in 0..nums.len() {
            let a = nums[i];

            // Skip processing positive numbers and duplicate elements
            if a > 0 {
                break;
            }

            if i > 0 && a == nums[i - 1] {
                continue; // Avoid duplicates
            }

            // Use two-pointer approach to find zero-sum triplets
            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                let three_sum = a + nums[l] + nums[r];

                if three_sum > 0 {
                    r -= 1; // Move the right pointer to the left
                } else if three_sum < 0 {
                    l += 1; // Move the left pointer to the right
                } else {
                    // Add the valid triplet to the result
                    res.push(vec![a, nums[l], nums[r]]);

                    // Avoid duplicate elements for the left pointer
                    l += 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }

                    // Avoid duplicate elements for the right pointer
                    r -= 1;
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                }
            }
        }

        res // Return the resulting triplets
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum(nums);

        // Expected output as a set to avoid order issues
        assert!(result.iter().any(|r| r == &vec![-1, -1, 2]));
        assert!(result.iter().any(|r| r == &vec![-1, 0, 1]));
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 1, 1];
        let result = Solution::three_sum(nums);

        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![0, 0, 0];
        let result = Solution::three_sum(nums);

        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }
}




