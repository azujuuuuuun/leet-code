struct Solution {}

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut min = 151;
        let mut exists = false;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                for k in 0..nums.len() {
                    if i < j
                        && j < k
                        && nums[i] < nums[j]
                        && nums[k] < nums[j]
                        && nums[i] + nums[j] + nums[k] < min
                    {
                        min = nums[i] + nums[j] + nums[k];
                        exists = true;
                    }
                }
            }
        }
        if exists {
            min
        } else {
            -1
        }
    }
}

fn main() {
    println!("{} {}", Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
    println!("{} {}", Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
    println!("{} {}", Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
}
