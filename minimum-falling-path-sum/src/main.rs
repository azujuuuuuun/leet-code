struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut path_sum = vec![];
        matrix.iter().for_each(|row| {
            let mut vec = vec![];
            row.iter().for_each(|cell| {
                vec.push(*cell);
            });
            path_sum.push(vec);
        });

        let n = matrix.len();
        for i in 1..n {
            for j in 0..n {
                path_sum[i][j] += path_sum[i - 1][j];
                if j as i32 - 1 >= 0 {
                    if matrix[i][j] + path_sum[i - 1][j - 1] < path_sum[i][j] {
                        path_sum[i][j] = matrix[i][j] + path_sum[i - 1][j - 1];
                    }
                }
                if j + 1 < n {
                    if matrix[i][j] + path_sum[i - 1][j + 1] < path_sum[i][j] {
                        path_sum[i][j] = matrix[i][j] + path_sum[i - 1][j + 1];
                    }
                }
            }
        }

        let mut min = path_sum[n - 1][0];
        for i in 1..n {
            let cell = path_sum[n - 1][i];
            if cell < min {
                min = cell;
            }
        }

        min
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]),
        13
    );
    println!(
        "{} {}",
        Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]),
        -59
    );
}
