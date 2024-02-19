struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            n == 2_i32.pow(n.ilog2())
        }
    }
}

fn main() {
    println!("{} {}", Solution::is_power_of_two(1), true);
    println!("{} {}", Solution::is_power_of_two(16), true);
    println!("{} {}", Solution::is_power_of_two(3), false);
}
