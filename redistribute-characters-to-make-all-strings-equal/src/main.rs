use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut hash_map = HashMap::new();
        words.iter().for_each(|word| {
            word.chars().for_each(|char| {
                match hash_map.get(&char) {
                    Some(count) => hash_map.insert(char, count + 1),
                    None => hash_map.insert(char, 1),
                };
            });
        });
        let words_num = words.len();
        let mut equal = true;
        hash_map.iter().for_each(|(_, count)| {
            if count % words_num != 0 {
                equal = false;
            }
        });
        equal
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]),
        true
    );
    println!(
        "{} {}",
        Solution::make_equal(vec!["ab".to_string(), "a".to_string(),]),
        false
    );
}
