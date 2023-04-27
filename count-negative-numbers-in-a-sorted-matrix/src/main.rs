struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        grid.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                if *cell < 0 {
                    count += 1;
                }
            })
        });

        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ])
    );
    println!(
        "{}",
        Solution::count_negatives(vec![vec![3, 2], vec![1, 0]])
    );
}
