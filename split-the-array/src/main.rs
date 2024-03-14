use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        nums.iter().for_each(|n| {
            if let Some(count) = map.get(n) {
                map.insert(*n, *count + 1);
            } else {
                map.insert(*n, 1);
            }
        });
        let mut is_possible = true;
        map.values().into_iter().for_each(|count| {
            if *count >= 3 {
                is_possible = false;
            }
        });
        is_possible
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]),
        true
    );
    println!(
        "{} {}",
        Solution::is_possible_to_split(vec![1, 1, 1, 1]),
        false
    );
}
