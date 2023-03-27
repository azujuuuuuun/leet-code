use std::{cmp::min, vec};

struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut copy_grid = grid.clone();

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                } else if i == 0 {
                    copy_grid[i][j] += copy_grid[i][j - 1];
                } else if j == 0 {
                    copy_grid[i][j] += copy_grid[i - 1][j];
                } else {
                    copy_grid[i][j] += min(copy_grid[i - 1][j], copy_grid[i][j - 1]);
                }
            }
        }

        copy_grid[m - 1][n - 1]
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
    );
}
