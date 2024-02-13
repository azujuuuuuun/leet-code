struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let mut palindrome = "".to_string();
        for i in 0..words.len() {
            if Solution::is_palindrome(&words[i]) {
                palindrome = words[i].to_string();
                break;
            }
        }
        palindrome
    }

    fn is_palindrome(s: &String) -> bool {
        let mut palindrome = true;
        for i in 0..s.len() {
            if s[i..i + 1] != s[s.len() - 1 - i..s.len() - i] {
                palindrome = false;
            }
        }
        palindrome
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::first_palindrome(vec![
            "abc".to_string(),
            "car".to_string(),
            "ada".to_string(),
            "racecar".to_string(),
            "cool".to_string()
        ]),
        "ada".to_string()
    );
    println!(
        "{} {}",
        Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
        "racecar".to_string()
    );
    println!(
        "{} {}",
        Solution::first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
        "".to_string()
    );
}
