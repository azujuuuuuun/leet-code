use std::borrow::Borrow;

struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max = 0;
        for i in 1..s.len() {
            let (left, right) = s.split_at(i);
            let zeros = left.chars().filter(|c| c.eq('0'.borrow())).count();
            let ones = right.chars().filter(|c| c.eq('1'.borrow())).count();
            if zeros + ones > max {
                max = zeros + ones;
            }
        }
        max as i32
    }
}

fn main() {
    println!("{} {}", Solution::max_score("011101".to_string()), 5);
    println!("{} {}", Solution::max_score("00111".to_string()), 5);
    println!("{} {}", Solution::max_score("1111".to_string()), 3);
}
