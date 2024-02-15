struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut sorted = nums.clone();
        sorted.sort();
        let mut sums = vec![];
        let mut sum: i64 = 0;
        sorted.iter().for_each(|n| {
            sum += *n as i64;
            sums.push(sum);
        });
        let mut perimeter: i64 = -1;
        for i in (1..sorted.len()).rev() {
            if sums[i - 1] > sorted[i] as i64 {
                perimeter = sums[i];
                break;
            }
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
