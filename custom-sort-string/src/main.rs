use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut map = HashMap::new();
        s.chars().into_iter().for_each(|c| {
            if let Some(count) = map.get(&c) {
                map.insert(c, count + 1);
            } else {
                map.insert(c, 1);
            }
        });
        let mut sorted = "".to_string();
        order.chars().into_iter().for_each(|c| {
            if let Some(count) = map.get(&c) {
                sorted += &c.to_string().repeat(*count as usize);
                map.remove(&c);
            }
        });
        map.iter().for_each(|(c, count)| {
            sorted += &c.to_string().repeat(*count as usize);
        });
        sorted
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
        "cbad".to_string()
    );
    println!(
        "{} {}",
        Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string()),
        "bcad".to_string()
    );
}
