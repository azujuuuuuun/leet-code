struct Solution {}

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let score = nums[0] + nums[1];
        let mut ops = 1;
        let mut i = 2;
        loop {
            if i + 1 > nums.len() - 1 {
                break;
            }
            if nums[i] + nums[i + 1] == score {
                ops += 1;
                i += 2;
            } else {
                break;
            }
        }
        ops
    }
}

fn main() {
    println!("{} {}", Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
    println!("{} {}", Solution::max_operations(vec![3, 2, 6, 1, 4]), 1);
}
