use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();

        stones.chars().into_iter().for_each(|c| {
            let count = map.get(&c);
            match count {
                Some(cnt) => {
                    map.insert(c, cnt + 1);
                }
                None => {
                    map.insert(c, 1);
                }
            }
        });

        let mut num_jewels = 0;
        jewels.chars().into_iter().for_each(|c| {
            let count = map.get(&c);
            match count {
                Some(cnt) => {
                    num_jewels += cnt;
                }
                None => {}
            }
        });

        num_jewels
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string())
    );
    println!(
        "{}",
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string())
    );
}
