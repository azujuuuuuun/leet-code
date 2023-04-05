struct Solution {}

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let first_1_index = nums.iter().position(|n| *n == 1);
        if first_1_index.is_none() {
            return true;
        }

        let mut index = first_1_index.unwrap() + 1;
        let mut cnt = 0;
        while index < nums.len() {
            if nums[index] == 0 {
                cnt += 1;
            } else if cnt < k {
                return false;
            } else {
                cnt = 0;
            }
            index += 1;
        }

        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2)
    );
    println!("{}", Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
    println!("{}", Solution::k_length_apart(vec![0, 0, 0], 2));
}
