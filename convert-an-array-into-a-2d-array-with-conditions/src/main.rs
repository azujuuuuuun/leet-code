use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        nums.iter().for_each(|n| match map.get(n) {
            Some(count) => {
                map.insert(*n, count + 1);
            }
            None => {
                map.insert(*n, 1);
            }
        });
        let mut array = vec![];
        loop {
            if map.is_empty() {
                break;
            }
            let mut row = vec![];
            let cloned_map = map.clone();
            let keys = cloned_map.keys();
            for n in keys {
                match map.get(n) {
                    Some(count) => {
                        if *count == 1 {
                            row.push(*n);
                            map.remove(n);
                        } else {
                            row.push(*n);
                            map.insert(*n, count - 1);
                        }
                    }
                    None => {}
                }
            }
            array.push(row);
        }
        array
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
        vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]]
    );
    println!(
        "{:?} {:?}",
        Solution::find_matrix(vec![1, 2, 3, 4]),
        vec![vec![4, 3, 2, 1]]
    );
}
