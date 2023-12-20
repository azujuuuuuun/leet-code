use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut hash_map = HashMap::new();
        grid.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                match hash_map.get(cell) {
                    Some(count) => hash_map.insert(cell, count + 1),
                    None => hash_map.insert(cell, 1),
                };
            });
        });
        let n = grid.len() as i32;
        let max = n * n;
        let mut twice = 0;
        let mut missing = 0;
        for num in 1..max + 1 {
            match hash_map.get(&num) {
                Some(count) => {
                    if *count == 2 {
                        twice = num;
                    }
                }
                None => missing = num,
            }
        }
        vec![twice, missing]
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
        vec![2, 4]
    );
    println!(
        "{:?} {:?}",
        Solution::find_missing_and_repeated_values(vec![
            vec![9, 1, 7],
            vec![8, 9, 2],
            vec![3, 4, 6]
        ]),
        vec![9, 5]
    );
}
