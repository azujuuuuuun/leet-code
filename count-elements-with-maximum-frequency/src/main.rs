use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut max = 1;
        let mut map = HashMap::new();
        nums.iter().for_each(|n| {
            if let Some(count) = map.get(n) {
                if count + 1 > max {
                    max = count + 1;
                }
                map.insert(*n, count + 1);
            } else {
                map.insert(*n, 1);
            }
        });

        let mut freq = 0;
        map.iter().for_each(|(_, v)| {
            if *v == max {
                freq += *v;
            }
        });

        freq
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]),
        4
    );
    println!(
        "{} {}",
        Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]),
        5
    );
}
