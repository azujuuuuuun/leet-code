struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        nums.iter().for_each(|n| {
            if count == 0 {
                candidate = *n;
            }
            if *n == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        });
        candidate
    }
}

fn main() {
    println!("{} {}", Solution::majority_element(vec![3, 2, 3]), 3);
    println!(
        "{} {}",
        Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]),
        2
    );
}
