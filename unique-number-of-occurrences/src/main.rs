use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        arr.iter().for_each(|n| {
            if map.contains_key(n) {
                map.insert(n, map.get(n).unwrap() + 1);
            } else {
                map.insert(n, 1);
            }
        });

        let mut set = HashSet::new();
        map.values().for_each(|c| {
            set.insert(c);
        });

        map.len() == set.len()
    }
}

fn main() {
    println!("{}", Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    println!("{}", Solution::unique_occurrences(vec![1, 2]));
    println!(
        "{}",
        Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0])
    );
}
