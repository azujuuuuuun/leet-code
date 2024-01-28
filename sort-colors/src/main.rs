struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] > nums[j] {
                    let tmp = nums[i];
                    nums[i] = nums[j];
                    nums[j] = tmp;
                }
            }
        }
    }
}

fn main() {
    let mut nums1 = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums1);
    println!("{:?}", nums1);
    let mut nums2 = vec![2, 0, 1];
    Solution::sort_colors(&mut nums2);
    println!("{:?}", nums2);
}
