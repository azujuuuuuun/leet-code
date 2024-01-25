use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        for _ in 0..text1.len() + 1 {
            let mut vec = vec![];
            for _ in 0..text2.len() + 1 {
                vec.push(0);
            }
            dp.push(vec);
        }

        for i in 1..text1.len() + 1 {
            for j in 1..text2.len() + 1 {
                if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i][j - 1], dp[i - 1][j]);
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
        3
    );
    println!(
        "{} {}",
        Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
        3
    );
    println!(
        "{} {}",
        Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
        0
    );
    println!(
        "{} {}",
        Solution::longest_common_subsequence("aaaaa".to_string(), "aaa".to_string()),
        3
    );
}
