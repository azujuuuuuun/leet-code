use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n + 1];

        for start in (0..n).rev() {
            let mut max = 0;
            let end = cmp::min(n, start + k as usize);

            for i in start..end {
                max = cmp::max(max, arr[i]);
                dp[start] = cmp::max(dp[start], dp[i + 1] + max * (i - start + 1) as i32);
            }
        }

        dp[0]
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
        84
    );
    println!(
        "{} {}",
        Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
        83
    );
    println!("{} {}", Solution::max_sum_after_partitioning(vec![1], 1), 1);
}
