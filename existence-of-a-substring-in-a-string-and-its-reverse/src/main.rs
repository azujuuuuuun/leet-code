struct Solution {}

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.reverse();
        let reversed = chars.iter().collect::<String>();
        let mut is_substring = false;
        for i in 0..s.len() - 1 {
            let substring = s[i..i + 2].to_string();
            if reversed.contains(&substring) {
                is_substring = true;
            }
        }
        is_substring
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::is_substring_present("leetcode".to_string()),
        true
    );
    println!(
        "{} {}",
        Solution::is_substring_present("abcba".to_string()),
        true
    );
    println!(
        "{} {}",
        Solution::is_substring_present("abcd".to_string()),
        false
    );
}
