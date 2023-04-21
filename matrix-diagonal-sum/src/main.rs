struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;

        mat.iter().enumerate().for_each(|(i, _)| {
            sum += mat[i][i];

            if mat.len() % 2 == 0 || i != mat.len() / 2 {
                sum += mat[i][mat.len() - 1 - i];
            }
        });

        sum
    }
}

fn main() {
    println!(
        "{}",
        Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    println!(
        "{}",
        Solution::diagonal_sum(vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1]
        ])
    );
    println!("{}", Solution::diagonal_sum(vec![vec![5]]));
}
