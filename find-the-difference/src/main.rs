struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut diff = t;
        s.chars().into_iter().for_each(|c| {
            let index = diff.find(c).unwrap();
            diff.remove(index);
        });
        return diff.chars().nth(0).unwrap();
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
    );
    println!(
        "{}",
        Solution::find_the_difference("".to_string(), "y".to_string())
    );
}
