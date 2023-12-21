struct Solution {}

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut xs = points
            .iter()
            .map(|p| p.get(0).unwrap().to_owned())
            .collect::<Vec<_>>();
        xs.sort();
        let mut max = 0;
        let mut current = xs.get(0).unwrap().to_owned();
        xs.iter().for_each(|x| {
            if *x != current {
                let diff = x - current;
                if diff > max {
                    max = diff;
                }
                current = *x;
            }
        });
        max
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::max_width_of_vertical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]]),
        1
    );
    println!(
        "{} {}",
        Solution::max_width_of_vertical_area(vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8]
        ]),
        3
    );
}
