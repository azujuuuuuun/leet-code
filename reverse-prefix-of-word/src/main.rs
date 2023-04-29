struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let ch_index = word.find(ch);
        match ch_index {
            Some(i) => {
                let mut chars = word[..i + 1].chars().collect::<Vec<_>>();
                chars.reverse();
                let prefix = chars
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                let rest = &word[i + 1..];

                prefix + rest
            }
            None => word,
        }
    }
}

fn main() {
    println!("{}", Solution::reverse_prefix("abcdefd".to_string(), 'd'));
    println!("{}", Solution::reverse_prefix("xyxzxe".to_string(), 'z'));
}
