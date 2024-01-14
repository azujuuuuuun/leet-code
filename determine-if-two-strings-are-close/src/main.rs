use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut word1_map = HashMap::new();
        let mut word2_map = HashMap::new();
        word1.chars().into_iter().for_each(|c| {
            if let Some(count) = word1_map.get(&c) {
                word1_map.insert(c, count + 1);
            } else {
                word1_map.insert(c, 1);
            }
        });
        word2.chars().into_iter().for_each(|c| {
            if let Some(count) = word2_map.get(&c) {
                word2_map.insert(c, count + 1);
            } else {
                word2_map.insert(c, 1);
            }
        });

        if word1_map.keys().len() != word2_map.keys().len() {
            return false;
        }
        let mut is_close = true;
        word1_map.keys().into_iter().for_each(|c| {
            if let None = word2_map.get(&c) {
                is_close = false;
            }
        });
        if !is_close {
            return is_close;
        }

        let mut word1_count: Vec<i32> = word1_map.values().into_iter().map(|v| *v).collect();
        let mut word2_count: Vec<i32> = word2_map.values().into_iter().map(|v| *v).collect();
        word1_count.sort();
        word2_count.sort();
        for i in 0..word1_count.len() {
            if word1_count[i] != word2_count[i] {
                is_close = false;
            }
        }

        is_close
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::close_strings("abc".to_string(), "bca".to_string()),
        true
    );
    println!(
        "{} {}",
        Solution::close_strings("a".to_string(), "aa".to_string()),
        false
    );
    println!(
        "{} {}",
        Solution::close_strings("cabbba".to_string(), "abbccc".to_string()),
        true
    );
}
