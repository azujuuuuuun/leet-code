struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let sum = n * (n + 1) / 2;
        let mut tmp = 0;
        for i in 1..n + 1 {
            tmp += i;
            if sum - tmp + i == tmp {
                return i;
            }
        }
        -1
    }
}

fn main() {
    println!("{} {}", Solution::pivot_integer(8), 6);
    println!("{} {}", Solution::pivot_integer(1), 1);
    println!("{} {}", Solution::pivot_integer(4), -1);
}
