// LeetCode 42. Trapping Rain Water

//Given n non-negative integers representing an elevation map 
//where the width of each bar is 1, compute how much water it 
//can trap after raining.


struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut left_max = height[l];
        let mut right_max = height[r];
        let mut res = 0;

        while l < r {
            if left_max < right_max {
                l += 1;
                left_max = std::cmp::max(left_max, height[l]);
                res += left_max - height[l];
            } else {
                r -= 1;
                right_max = std::cmp::max(right_max, height[r]);
                res += right_max - height[r];
            }
        }

        res
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
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let result = Solution::trap(height);
        assert_eq!(result, 6); // Expected output is 6
    }

    #[test]
    fn test_example_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let result = Solution::trap(height);
        assert_eq!(result, 9); // Expected output is 9
    }

    #[test]
    fn test_empty_height() {
        let height = vec![];
        let result = Solution::trap(height);
        assert_eq!(result, 0); // No height, no trapped water
    }

    #[test]
    fn test_no_trapped_water() {
        let height = vec![1, 2, 3, 4, 5]; // No valleys
        let result = Solution::trap(height);
        assert_eq!(result, 0); // Expected output is 0
    }
}
