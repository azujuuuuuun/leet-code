struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut array = vec![0; nums.len()];
        let mut positive_index = 0;
        let mut negative_index = 1;
        for i in 0..nums.len() {
            if nums[i] > 0 {
                array[positive_index] = nums[i];
                positive_index += 2;
            } else {
                array[negative_index] = nums[i];
                negative_index += 2;
            }
        }
        array
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
        vec![3, -2, 1, -5, 2, -4]
    );
    println!(
        "{:?} {:?}",
        Solution::rearrange_array(vec![-1, 1]),
        vec![1, -1]
    );
}
