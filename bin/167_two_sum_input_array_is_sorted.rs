// LeetCode 167 - Two Sum - Input Array is Sorted

// Given a 1-indexed array of integers numbers that is already 
// sorted in non-decreasing order, find two numbers such that they 
// add up to a specific target number. Let these two numbers be 
// numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.

// Return the indices of the two numbers, index1 and index2, 
// added by one as an integer array [index1, index2] of length 2.

// The tests are generated such that there is exactly one solution. 
// You may not use the same element twice.

// Your solution must use only constant extra space.

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);

        while l < r {
            let cur_sum = numbers[l] + numbers[r];

            if cur_sum > target {
                r -= 1;
            } else if cur_sum < target {
                l += 1;
            } else {
                return vec![l as i32 + 1, r as i32 + 1]; // Returning 1-based indices
            }
        }

        vec![] // Return empty vector if no solution is found
    }
}

fn main() {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 2]; // The expected output
        let result = Solution::two_sum(numbers, target);
        assert_eq!(result, expected); // Assert the result matches the expected output
    }

    #[test]
    fn test_example2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let expected = vec![1, 3]; // The expected output
        let result = Solution::two_sum(numbers, target);
        assert_eq!(result, expected); // Assert the result matches the expected output
    }

    #[test]
    fn test_example3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let expected = vec![1, 2]; // The expected output
        let result = Solution::two_sum(numbers, target);
        assert_eq!(result, expected); // Assert the result matches the expected output
    }
}
