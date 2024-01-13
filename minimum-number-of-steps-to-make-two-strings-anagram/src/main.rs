use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_char_counter = HashMap::new();
        let mut t_char_counter = HashMap::new();
        s.chars().into_iter().for_each(|c| {
            match s_char_counter.get(&c) {
                Some(count) => s_char_counter.insert(c, count + 1),
                None => s_char_counter.insert(c, 1),
            };
        });
        t.chars().into_iter().for_each(|c| {
            match t_char_counter.get(&c) {
                Some(count) => t_char_counter.insert(c, count + 1),
                None => t_char_counter.insert(c, 1),
            };
        });

        let mut steps = 0;
        s_char_counter.iter().for_each(|(k, v)| {
            match t_char_counter.get(&k) {
                Some(count) => {
                    if count < v {
                        steps += v - count;
                    }
                }
                None => {
                    steps += v;
                }
            };
        });
        steps
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::min_steps("bab".to_string(), "aba".to_string()),
        1
    );
    println!(
        "{} {}",
        Solution::min_steps("leetcode".to_string(), "practice".to_string()),
        5
    );
    println!(
        "{} {}",
        Solution::min_steps("anagram".to_string(), "mangaar".to_string()),
        0
    );
}
