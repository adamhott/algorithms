// LeedCode 200. Number of Islands

// Given an m x n 2D binary grid grid which represents a map 
// of '1's (land) and '0's (water), return the number of islands.

// An island is surrounded by water and is formed by connecting 
// adjacent lands horizontally or vertically. You may assume all 
// four edges of the grid are all surrounded by water.

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let mut islands = 0;
        let mut visit = HashSet::new();
        let rows = grid.len();
        let cols = grid[0].len();

        fn dfs(r: i32, c: i32, grid: &Vec<Vec<char>>, visit: &mut HashSet<(i32, i32)>, rows: usize, cols: usize) {
            if r < 0 || c < 0 || r >= rows as i32 || c >= cols as i32 || grid[r as usize][c as usize] == '0' || visit.contains(&(r, c)) {
                return;
            }

            visit.insert((r, c));
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for &(dr, dc) in directions.iter() {
                dfs(r + dr, c + dc, grid, visit, rows, cols);
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' && !visit.contains(&(r as i32, c as i32)) {
                    islands += 1;
                    dfs(r as i32, c as i32, &grid, &mut visit, rows, cols);
                }
            }
        }

        islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<Vec<char>> = vec![];
        assert_eq!(Solution::num_islands(grid), 0);
    }

    #[test]
    fn test_no_land() {
        let grid = vec![
            vec!['0', '0', '0'],
            vec!['0', '0', '0'],
            vec!['0', '0', '0']
        ];
        assert_eq!(Solution::num_islands(grid), 0);
    }
}
