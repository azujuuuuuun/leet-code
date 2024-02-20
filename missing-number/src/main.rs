struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut missing = (nums.len() as i32) * (nums.len() as i32 + 1) / 2;
        nums.iter().for_each(|n| {
            missing -= *n;
        });
        missing
    }
}

fn main() {
    println!("{} {}", Solution::missing_number(vec![3, 0, 1]), 2);
    println!("{} {}", Solution::missing_number(vec![0, 1]), 2);
    println!(
        "{} {}",
        Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
        8
    );
}
