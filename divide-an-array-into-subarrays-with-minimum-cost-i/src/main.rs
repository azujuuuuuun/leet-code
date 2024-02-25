struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let min1 = nums[0];
        let mut min2 = if nums[1] <= nums[2] { nums[1] } else { nums[2] };
        let mut min3 = if nums[1] <= nums[2] { nums[2] } else { nums[1] };
        for i in 3..nums.len() {
            let n = nums[i];
            if n < min2 {
                min3 = min2;
                min2 = n;
            } else if n < min3 {
                min3 = n;
            }
        }
        min1 + min2 + min3
    }
}

fn main() {
    println!("{} {}", Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
    println!("{} {}", Solution::minimum_cost(vec![5, 4, 3]), 12);
    println!("{} {}", Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
}
