use std::cmp;

struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![10001; (n + 1) as usize];
        dp[0] = 0;
        for i in 1..(n + 1) {
            let mut j = 1;
            loop {
                if j * j > i {
                    break;
                }
                dp[i as usize] = cmp::min(dp[i as usize], dp[(i - j * j) as usize] + 1);
                j += 1;
            }
        }
        dp[n as usize]
    }
}

fn main() {
    println!("{} {}", Solution::num_squares(12), 3);
    println!("{} {}", Solution::num_squares(13), 2);
}
