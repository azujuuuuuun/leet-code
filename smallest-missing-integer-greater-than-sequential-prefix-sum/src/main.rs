use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..nums.len() {
            if i == 0 {
                sum += nums[i];
            } else if nums[i] == nums[i - 1] + 1 {
                sum += nums[i];
            } else {
                break;
            }
        }
        let mut hash_set = HashSet::new();
        nums.iter().for_each(|n| {
            hash_set.insert(*n);
        });
        loop {
            if let Some(_) = hash_set.get(&sum) {
                sum += 1;
            } else {
                break;
            }
        }
        sum
    }
}

fn main() {
    println!("{} {}", Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
    println!(
        "{} {}",
        Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]),
        15
    );
}
