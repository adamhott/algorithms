// Leetcode 36. Valid Sudoku
//Determine if a 9 x 9 Sudoku board is valid. 
//Only the filled cells need to be validated 
//according to the following rules:

//Each row must contain the digits 1-9 without repetition.
//Each column must contain the digits 1-9 without repetition.
//Each of the nine 3 x 3 sub-boxes of the grid must 
//contain the digits 1-9 without repetition.

//Note:

//A Sudoku board (partially filled) could be
// valid but is not necessarily solvable.
//Only the filled cells need to be validated 
//according to the mentioned rules.

use std::collections::HashSet;
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];
                if val == '.' {
                    continue;
                }

                let square_key = (r / 3, c / 3);

                // Check row
                if rows.entry(r).or_default().contains(&val) {
                    return false;
                }
                // Check column
                if cols.entry(c).or_default().contains(&val) {
                    return false;
                }
                // Check square
                if squares.entry(square_key).or_default().contains(&val) {
                    return false;
                }

                // Insert the character into respective sets
                rows.get_mut(&r).unwrap().insert(val);
                cols.get_mut(&c).unwrap().insert(val);
                squares.get_mut(&square_key).unwrap().insert(val);
            }
        }

        true
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_sudoku() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_sudoku() {
        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}



