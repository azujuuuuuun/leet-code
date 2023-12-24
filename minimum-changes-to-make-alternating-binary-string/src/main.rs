struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut start_with_0 = "".to_string();
        let mut start_with_1 = "".to_string();
        for i in 0..s.len() {
            if i % 2 == 0 {
                start_with_0 += "0";
                start_with_1 += "1";
            } else {
                start_with_0 += "1";
                start_with_1 += "0";
            }
        }
        let mut num_0_start = 0;
        let mut num_1_start = 0;
        for i in 0..s.len() {
            if s.chars().nth(i).unwrap() != start_with_0.chars().nth(i).unwrap() {
                num_0_start += 1;
            }
            if s.chars().nth(i).unwrap() != start_with_1.chars().nth(i).unwrap() {
                num_1_start += 1;
            }
        }
        if num_0_start < num_1_start {
            num_0_start
        } else {
            num_1_start
        }
    }
}

fn main() {
    println!("{} {}", Solution::min_operations("0100".to_string()), 1);
    println!("{} {}", Solution::min_operations("10".to_string()), 0);
    println!("{} {}", Solution::min_operations("1111".to_string()), 2);
    println!("{} {}", Solution::min_operations("10010100".to_string()), 3);
}
