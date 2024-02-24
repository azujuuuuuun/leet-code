use std::cmp;

struct Solution {}

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut column_max = vec![0; matrix[0].len()];
        for i in 0..matrix[0].len() {
            for j in 0..matrix.len() {
                column_max[i] = cmp::max(column_max[i], matrix[j][i]);
            }
        }
        let mut ans = vec![];
        for i in 0..matrix.len() {
            let mut vec = vec![];
            for j in 0..matrix[i].len() {
                if matrix[i][j] == -1 {
                    vec.push(column_max[j]);
                } else {
                    vec.push(matrix[i][j]);
                }
            }
            ans.push(vec);
        }
        ans
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::modified_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]),
        vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]]
    );
    println!(
        "{:?} {:?}",
        Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]]),
        vec![vec![3, 2], vec![5, 2]]
    );
}
