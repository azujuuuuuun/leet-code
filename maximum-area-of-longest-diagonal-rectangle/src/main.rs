struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut max_diagonal = 0_f64;
        dimensions.iter().for_each(|d| {
            let length = d[0];
            let width = d[1];
            let diagonal = ((length * length + width * width) as f64).sqrt();
            let area = length * width;
            if diagonal > max_diagonal {
                max_diagonal = diagonal;
                max_area = area;
            } else if diagonal == max_diagonal {
                if area >= max_area {
                    max_area = area;
                }
            }
        });
        max_area
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]),
        48
    );
    println!(
        "{} {}",
        Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]),
        12
    );
    println!(
        "{} {}",
        Solution::area_of_max_diagonal(vec![
            vec![4, 7],
            vec![8, 9],
            vec![5, 3],
            vec![6, 10],
            vec![2, 9],
            vec![3, 10],
            vec![2, 2],
            vec![5, 8],
            vec![5, 10],
            vec![5, 6],
            vec![8, 9],
            vec![10, 7],
            vec![8, 9],
            vec![3, 7],
            vec![2, 6],
            vec![5, 1],
            vec![7, 4],
            vec![1, 10],
            vec![1, 7],
            vec![6, 9],
            vec![3, 3],
            vec![4, 6],
            vec![8, 2],
            vec![10, 6],
            vec![7, 9],
            vec![9, 2],
            vec![1, 2],
            vec![3, 8],
            vec![10, 2],
            vec![4, 1],
            vec![9, 7],
            vec![10, 3],
            vec![6, 9],
            vec![9, 8],
            vec![7, 7],
            vec![5, 7],
            vec![5, 4],
            vec![6, 5],
            vec![1, 8],
            vec![2, 3],
            vec![7, 10],
            vec![3, 9],
            vec![5, 7],
            vec![2, 4],
            vec![5, 6],
            vec![9, 5],
            vec![8, 8],
            vec![8, 10],
            vec![6, 8],
            vec![5, 1],
            vec![10, 8],
            vec![7, 4],
            vec![2, 1],
            vec![2, 7],
            vec![10, 3],
            vec![2, 5],
            vec![7, 6],
            vec![10, 5],
            vec![10, 9],
            vec![5, 7],
            vec![10, 6],
            vec![4, 3],
            vec![10, 4],
            vec![1, 5],
            vec![8, 9],
            vec![3, 1],
            vec![2, 5],
            vec![9, 10],
            vec![6, 6],
            vec![5, 10],
            vec![10, 2],
            vec![6, 10],
            vec![1, 1],
            vec![8, 6],
            vec![1, 7],
            vec![6, 3],
            vec![9, 3],
            vec![1, 4],
            vec![1, 1],
            vec![10, 4],
            vec![7, 9],
            vec![4, 5],
            vec![2, 8],
            vec![7, 9],
            vec![7, 3],
            vec![4, 9],
            vec![2, 8],
            vec![4, 6],
            vec![9, 1],
            vec![8, 4],
            vec![2, 4],
            vec![7, 8],
            vec![3, 5],
            vec![7, 6],
            vec![8, 6],
            vec![4, 7],
            vec![25, 60],
            vec![39, 52],
            vec![16, 63],
            vec![33, 56]
        ]),
        2028
    );
    println!(
        "{} {}",
        Solution::area_of_max_diagonal(vec![
            vec![6, 5],
            vec![8, 6],
            vec![2, 10],
            vec![8, 1],
            vec![9, 2],
            vec![3, 5],
            vec![3, 5]
        ]),
        20
    );
}
