use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        s.chars().into_iter().for_each(|c| {
            if let Some(count) = map.get(&c) {
                map.insert(c, *count + 1);
            } else {
                map.insert(c, 1);
            }
        });
        let mut answer = -1;
        s.chars().into_iter().enumerate().for_each(|(i, c)| {
            if let Some(count) = map.get(&c) {
                if answer == -1 && *count == 1 {
                    answer = i as i32;
                }
            }
        });
        answer
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::first_uniq_char("leetcode".to_string()),
        0
    );
    println!(
        "{} {}",
        Solution::first_uniq_char("loveleetcode".to_string()),
        2
    );
    println!("{} {}", Solution::first_uniq_char("aabb".to_string()), -1);
}
