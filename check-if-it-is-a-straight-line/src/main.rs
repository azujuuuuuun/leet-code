struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut m = 0f64;
        let mut is_straight = true;

        for i in 0..coordinates.len() {
            if i == coordinates.len() - 1 {
                break;
            }

            let c1 = &coordinates[i];
            let x1 = c1[0];
            let y1 = c1[1];
            let c2 = &coordinates[i + 1];
            let x2 = c2[0];
            let y2 = c2[1];

            let tmp = if x2 - x1 == 0 {
                i32::MAX as f64
            } else {
                (y2 - y1) as f64 / (x2 - x1) as f64
            };

            if i == 0 {
                m = tmp;
                continue;
            }

            if tmp != m {
                is_straight = false;
                break;
            }

            m = tmp;
        }

        is_straight
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ])
    );
    println!(
        "{}",
        Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ])
    );
    println!(
        "{}",
        Solution::check_straight_line(vec![vec![0, 0], vec![0, 5], vec![5, 5], vec![5, 0],])
    );
    println!(
        "{}",
        Solution::check_straight_line(vec![
            vec![-4, -3],
            vec![1, 0],
            vec![3, -1],
            vec![0, -1],
            vec![-5, 2]
        ])
    );
}
