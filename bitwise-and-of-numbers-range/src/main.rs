struct Solution {}

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let left_ilog2 = if left == 0 { 0 } else { left.ilog2() };
        let right_ilog2 = if right == 0 { 0 } else { right.ilog2() };
        if left_ilog2 != right_ilog2 {
            return 0;
        }
        let mut ans = left;
        for n in left..right + 1 {
            ans &= n;
        }
        ans
    }
}

fn main() {
    // 8 4 2 1
    // 0 1 0 1 -> 5
    // 0 1 1 0 -> 6
    // 0 1 1 1 -> 7
    // 1 0 0 0 -> 8
    println!("{} {}", Solution::range_bitwise_and(5, 7), 4);
    println!("{} {}", Solution::range_bitwise_and(0, 0), 0);
    println!("{} {}", Solution::range_bitwise_and(1, 2147483647), 0);
}
