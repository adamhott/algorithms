// LeetCode 153. Find Minimum in Rotated Sorted Array

//Suppose an array of length n sorted in ascending order 
//is rotated between 1 and n times. For example, 
//the array nums = [0,1,2,4,5,6,7] might become:

//[4,5,6,7,0,1,2] if it was rotated 4 times.
//[0,1,2,4,5,6,7] if it was rotated 7 times.

//Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 
//time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

//Given the sorted rotated array nums of unique elements, return 
//the minimum element of this array.

//You must write an algorithm that runs in O(log n) time.

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut start = 0; // Start index
        let mut end = nums.len() as i32 - 1; // End index
        let mut curr_min = i32::MAX; // Initialize to the maximum possible integer value

        while start < end {
            let mid = start + (end - start) / 2; // Calculate the midpoint
            curr_min = curr_min.min(nums[mid as usize]); // Update the current minimum

            // Check if the right part has the minimum
            if nums[mid as usize] > nums[end as usize] {
                start = mid + 1; // Adjust the start index
            } else {
                end = mid - 1; // Adjust the end index
            }
        }

        // Return the minimum of the current minimum and the value at the start index
        curr_min.min(nums[start as usize])
    }
}


fn main() {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_min_example1() {
        let nums = vec![3, 4, 5, 1, 2];
        let expected_output = 1;
        let result = Solution::find_min(nums);
        assert_eq!(result, expected_output, "Test failed for example 1");
    }

    #[test]
    fn test_find_min_example2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let expected_output = 0;
        let result = Solution::find_min(nums);
        assert_eq!(result, expected_output, "Test failed for example 2");
    }

    #[test]
    fn test_find_min_example3() {
        let nums = vec![11, 13, 15, 17];
        let expected_output = 11;
        let result = Solution::find_min(nums);
        assert_eq!(result, expected_output, "Test failed for example 3");
    }
}



