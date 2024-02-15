struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut sorted = nums.clone();
        sorted.sort();
        let mut sum: i64 = 0;
        let mut perimeter: i64 = -1;
        for i in 0..sorted.len() {
            if i >= 2 && (sorted[i] as i64) < sum {
                perimeter = sum + sorted[i] as i64;
            }
            sum += sorted[i] as i64;
        }
        perimeter
    }
}

fn main() {
    println!("{} {}", Solution::largest_perimeter(vec![5, 5, 5]), 15);
    println!(
        "{} {}",
        Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]),
        12
    );
    println!("{} {}", Solution::largest_perimeter(vec![5, 5, 50]), -1);
}
