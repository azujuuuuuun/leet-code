struct Solution {}

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let zero_count = s.chars().filter(|c| c.eq(&'0')).count();
        let one_count = s.chars().filter(|c| c.eq(&'1')).count();
        "1".repeat(one_count - 1) + &"0".repeat(zero_count) + "1"
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::maximum_odd_binary_number("010".to_string()),
        "001".to_string()
    );
    println!(
        "{} {}",
        Solution::maximum_odd_binary_number("0101".to_string()),
        "1001".to_string()
    );
}
