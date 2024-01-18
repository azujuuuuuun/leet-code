struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut dp = vec![];
        for _ in 0..n + 1 {
            dp.push(0);
        }
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..(n as usize) + 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}

fn main() {
    println!("{} {}", Solution::climb_stairs(2), 2);
    println!("{} {}", Solution::climb_stairs(3), 3);
}
