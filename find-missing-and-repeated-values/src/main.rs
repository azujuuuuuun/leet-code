use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let max = n * n;
        let mut hash_map = HashMap::new();
        for num in 1..max + 1 {
            hash_map.insert(num as i32, 1);
        }
        grid.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                let count = hash_map.get(cell).unwrap();
                hash_map.insert(cell.to_owned(), count - 1);
            });
        });
        let twice_vec = hash_map
            .iter()
            .filter(|(_, v)| **v == -1)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        let twice = twice_vec.first().unwrap();
        let missing_vec = hash_map
            .iter()
            .filter(|(_, v)| **v == 1)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        let missing = missing_vec.first().unwrap();
        vec![**twice, **missing]
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
