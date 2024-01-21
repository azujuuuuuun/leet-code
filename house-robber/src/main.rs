struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        nums.iter().enumerate().for_each(|(i, n)| {
            if i == 0 {
                dp.push((0, *n));
            } else if i == 1 {
                dp.push((Self::max(dp[0].0, dp[0].1), dp[0].0 + *n));
            } else {
                dp.push((
                    Self::max(dp[i - 1].0, dp[i - 1].1),
                    Self::max(dp[i - 2].1, dp[i - 1].0) + *n,
                ));
            }
        });
        let (left, right) = dp.last().unwrap();
        Self::max(*left, *right)
    }

    fn max(num1: i32, num2: i32) -> i32 {
        if num1 > num2 {
            num1
        } else {
            num2
        }
    }
}

fn main() {
    println!("{} {}", Solution::rob(vec![1, 2, 3, 1]), 4);
    println!("{} {}", Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    println!("{} {}", Solution::rob(vec![2, 1, 1, 2]), 4);
}
