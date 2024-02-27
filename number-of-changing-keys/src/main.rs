struct Solution {}

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let s_lowercase = s.to_lowercase();
        let mut count = 0;
        for i in 0..s_lowercase.len() - 1 {
            if s_lowercase.chars().nth(i) != s_lowercase.chars().nth(i + 1) {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::count_key_changes("aAbBcC".to_string()),
        2
    );
    println!(
        "{} {}",
        Solution::count_key_changes("AaAaAaaA".to_string()),
        0
    );
}
