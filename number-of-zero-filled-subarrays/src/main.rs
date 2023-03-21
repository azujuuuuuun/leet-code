struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut subarrays = 0;
        let mut count = 0;

        nums.iter().for_each(|n| {
            if n.eq(&0) {
                count += 1;
                subarrays += count;
            } else {
                count = 0;
            }
        });

        subarrays
    }
}

fn main() {
    println!(
        "{}",
        Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])
    );
    println!("{}", Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]));
    println!("{}", Solution::zero_filled_subarray(vec![2, 10, 2019]));
}
