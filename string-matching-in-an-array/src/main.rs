use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut set = HashSet::new();

        words.iter().for_each(|w1| {
            words.iter().for_each(|w2| {
                if w1 == w2 {
                    return;
                }

                if w1.contains(w2) {
                    set.insert(w2.to_string());
                }
            })
        });

        let mut substrings = vec![];
        set.iter().for_each(|w| {
            substrings.push(w.to_string());
        });

        substrings
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string()
        ])
    );
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "leetcode".to_string(),
            "et".to_string(),
            "code".to_string(),
        ])
    );
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "blue".to_string(),
            "green".to_string(),
            "bu".to_string(),
        ])
    );
}
