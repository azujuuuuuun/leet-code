use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut is_pangram = true;

        "abcdefghijklmnopqrstuvwxyz".chars().for_each(|c| {
            if !sentence.contains(c) {
                is_pangram = false;
            }
        });

        is_pangram
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string())
    );
    println!("{}", Solution::check_if_pangram("leetcode".to_string()));
}
