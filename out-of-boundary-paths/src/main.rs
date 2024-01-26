struct Solution {}

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m_usize = m as usize;
        let n_usize = n as usize;
        let max_move_usize = max_move as usize;
        let start_row_usize = start_row as usize;
        let start_column_usize = start_column as usize;
        let modulo = 1000000007;

        let mut dp: Vec<Vec<Vec<i64>>> = vec![];
        for _ in 0..max_move + 1 {
            let mut grid = vec![];
            for _ in 0..m + 2 {
                let mut vec = vec![];
                for _ in 0..n + 2 {
                    vec.push(0);
                }
                grid.push(vec);
            }
            dp.push(grid);
        }
        dp[0][start_row_usize + 1][start_column_usize + 1] = 1;

        for mv in 1..max_move_usize + 1 {
            for i in 0..m_usize + 2 {
                for j in 0..n_usize + 2 {
                    if i == 0 || i == m_usize + 1 || j == 0 || j == n_usize + 1 {
                        let mut count = 0;
                        if i == 0 {
                            if j != 0 && j != n_usize + 1 {
                                count += dp[mv - 1][i + 1][j];
                            }
                        }
                        if i == m_usize + 1 {
                            if j != 0 && j != n_usize + 1 {
                                count += dp[mv - 1][i - 1][j];
                            }
                        }
                        if j == 0 {
                            if i != 0 && i != m_usize + 1 {
                                count += dp[mv - 1][i][j + 1];
                            }
                        }
                        if j == n_usize + 1 {
                            if i != 0 && i != m_usize + 1 {
                                count += dp[mv - 1][i][j - 1];
                            }
                        }
                        dp[mv][i][j] = dp[mv - 1][i][j] % modulo + count % modulo;
                    } else {
                        let mut count = 0;
                        count += if i == 1 { 0 } else { dp[mv - 1][i - 1][j] };
                        count += if i == m_usize {
                            0
                        } else {
                            dp[mv - 1][i + 1][j]
                        };
                        count += if j == 1 { 0 } else { dp[mv - 1][i][j - 1] };
                        count += if j == n_usize {
                            0
                        } else {
                            dp[mv - 1][i][j + 1]
                        };
                        dp[mv][i][j] = count % modulo;
                    }
                }
            }
        }

        let mut count = 0;
        for i in 0..n_usize + 2 {
            count += dp[max_move_usize][0][i] % modulo;
            count += dp[max_move_usize][m_usize + 1][i] % modulo;
        }
        for i in 0..m_usize + 2 {
            count += dp[max_move_usize][i][0] % modulo;
            count += dp[max_move_usize][i][n_usize + 1] % modulo;
        }
        (count % modulo) as i32
    }
}

fn main() {
    println!("{} {}", Solution::find_paths(2, 2, 2, 0, 0), 6);
    println!("{} {}", Solution::find_paths(1, 3, 3, 0, 1), 12);
    println!("{} {}", Solution::find_paths(8, 50, 23, 5, 26), 914783380);
    println!("{} {}", Solution::find_paths(36, 5, 50, 15, 3), 390153306);
}
