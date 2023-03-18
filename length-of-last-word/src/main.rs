struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(" ").last().unwrap_or("").len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_last_word(String::from("Hello world"))
    );
    println!(
        "{}",
        Solution::length_of_last_word(String::from("   fly me   to   the moon  "))
    );
    println!(
        "{}",
        Solution::length_of_last_word(String::from("luffy is still joyboy"))
    );
}
