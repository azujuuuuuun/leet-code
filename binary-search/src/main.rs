struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = -1;
        let mut right = nums.len() as i32;

        while right - left > 1 {
            let middle = left + (right - left) / 2;
            if nums[middle as usize] == target {
                return middle;
            } else if nums[middle as usize] < target {
                left = middle;
            } else {
                right = middle;
            }
        }

        -1
    }
}

fn main() {
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
}
