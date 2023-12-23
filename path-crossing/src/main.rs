use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut hash_set = HashSet::new();
        hash_set.insert(x.to_string() + "," + &y.to_string());
        let mut cross = false;
        path.chars().for_each(|d| {
            if d == 'N' {
                y += 1;
            } else if d == 'S' {
                y -= 1;
            } else if d == 'E' {
                x += 1;
            } else if d == 'W' {
                x -= 1;
            }
            let key = x.to_string() + "," + &y.to_string();
            match hash_set.get(&key) {
                Some(_) => cross = true,
                None => {}
            }
            hash_set.insert(key);
        });
        cross
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::is_path_crossing("NES".to_string()),
        false
    );
    println!(
        "{} {}",
        Solution::is_path_crossing("NESWW".to_string()),
        true
    );
}
