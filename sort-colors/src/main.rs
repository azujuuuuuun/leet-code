struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count0 = 0;
        let mut count1 = 0;
        let mut count2 = 0;
        nums.iter().for_each(|n| {
            if *n == 0 {
                count0 += 1;
            } else if *n == 1 {
                count1 += 1;
            } else {
                count2 += 1;
            }
        });
        for i in 0..(count0 + count1 + count2) {
            if i < count0 {
                nums[i] = 0;
            } else if i < count0 + count1 {
                nums[i] = 1;
            } else {
                nums[i] = 2;
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
