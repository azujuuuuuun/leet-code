struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;
        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                let c1 = s.get(i..i + 1).unwrap();
                let c2 = s.get(j..j + 1).unwrap();
                if c1 == c2 && (j - 1 - i) as i32 > max {
                    max = (j - 1 - i) as i32;
                }
            }
        }
        max
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::max_length_between_equal_characters("aa".to_string()),
        0
    );
    println!(
        "{} {}",
        Solution::max_length_between_equal_characters("abca".to_string()),
        2
    );
    println!(
        "{} {}",
        Solution::max_length_between_equal_characters("cbzxy".to_string()),
        -1
    );
}
