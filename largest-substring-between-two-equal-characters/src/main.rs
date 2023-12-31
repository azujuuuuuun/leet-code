use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;
        let mut hash_map = HashMap::new();
        s.chars().enumerate().for_each(|(i, c)| {
            match hash_map.get(&c) {
                Some(first_index) => {
                    if i as i32 - first_index - 1 > max {
                        max = i as i32 - first_index - 1;
                    }
                }
                None => {
                    hash_map.insert(c, i as i32);
                }
            };
        });
        max
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::max_length_between_equal_characters("aa".to_string()),
        0
    );
    println!(
        "{} {}",
        Solution::max_length_between_equal_characters("abca".to_string()),
        2
    );
    println!(
        "{} {}",
        Solution::max_length_between_equal_characters("cbzxy".to_string()),
        -1
    );
}
