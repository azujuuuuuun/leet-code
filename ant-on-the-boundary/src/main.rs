struct Solution {}

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut count = 0;
        nums.iter().for_each(|n| {
            current += *n;
            if current == 0 {
                count += 1;
            }
        });
        count
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::return_to_boundary_count(vec![2, 3, -5]),
        1
    );
    println!(
        "{} {}",
        Solution::return_to_boundary_count(vec![3, 2, -3, -4]),
        0
    );
}
