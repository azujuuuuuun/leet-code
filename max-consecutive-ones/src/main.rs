struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        nums.iter().for_each(|n| {
            if *n == 1 {
                count += 1;
                if count > max {
                    max = count;
                }
            } else {
                count = 0;
            }
        });
        max
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );
    println!(
        "{} {}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
        2
    );
}
