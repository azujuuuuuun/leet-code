struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max = 0;

        for i in 1..s.len() {
            let (left, right) = s.split_at(i);
            let zeros = left.chars().filter(|c| *c == '0').count();
            let ones = right.chars().filter(|c| *c == '1').count();
            max = if (zeros + ones) as i32 > max {
                (zeros + ones) as i32
            } else {
                max
            }
        }

        max
    }
}

fn main() {
    println!("{}", Solution::max_score("011101".to_string()));
    println!("{}", Solution::max_score("00111".to_string()));
    println!("{}", Solution::max_score("1111".to_string()));
}
