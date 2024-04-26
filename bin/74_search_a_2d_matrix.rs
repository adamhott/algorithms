// LeetCode 74. Search a 2D Matrix

//You are given an m x n integer matrix matrix with 
//the following two properties:

//Each row is sorted in non-decreasing order.
//The first integer of each row is greater than the 
//last integer of the previous row.
//Given an integer target, return true if target is 
//in matrix or false otherwise.

//You must write a solution in O(log(m * n)) time complexity.

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let (rows, cols) = (matrix.len(), matrix[0].len());
        let (mut top, mut bottom) = (0, rows as isize - 1);

        // Binary search to find the row where the target might exist
        while top <= bottom {
            let mid_row = (top + bottom) / 2;
            if target > matrix[mid_row as usize][cols - 1] {
                top = mid_row + 1;
            } else if target < matrix[mid_row as usize][0] {
                bottom = mid_row - 1;
            } else {
                break;
            }
        }

        // If the row range is invalid, the target doesn't exist in the matrix
        if top > bottom {
            return false;
        }

        let mid_row = (top + bottom) / 2;
        let (mut left, mut right) = (0, cols as isize - 1);

        // Binary search within the found row
        while left <= right {
            let mid_col = (left + right) / 2;
            if target > matrix[mid_row as usize][mid_col as usize] {
                left = mid_col + 1;
            } else if target < matrix[mid_row as usize][mid_col as usize] {
                right = mid_col - 1;
            } else {
                return true; // Target found
            }
        }

        false // Target not found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
        ];
        let target = 3;
        let expected_output = true; // The target is found in the matrix
        assert_eq!(Solution::search_matrix(matrix, target), expected_output);
    }

    #[test]
    fn test_case_2() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
        ];
        let target = 13;
        let expected_output = false; // The target is not in the matrix
        assert_eq!(Solution::search_matrix(matrix, target), expected_output);
    }
}