use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut not_robbed = 0;
        let mut robbed = 0;
        nums.iter().for_each(|n| {
            let new_not_robbed = max(not_robbed, robbed);
            let new_robbed = not_robbed + *n;
            not_robbed = new_not_robbed;
            robbed = new_robbed;
        });
        max(robbed, not_robbed)
    }
}

fn main() {
    println!("{} {}", Solution::rob(vec![1, 2, 3, 1]), 4);
    println!("{} {}", Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    println!("{} {}", Solution::rob(vec![2, 1, 1, 2]), 4);
}
