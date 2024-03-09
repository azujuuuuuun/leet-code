struct Solution {}

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i1 = 0;
        let mut i2 = 0;
        let mut common = -1;
        loop {
            if i1 >= nums1.len() || i2 >= nums2.len() {
                break;
            }
            if nums1[i1] == nums2[i2] {
                common = nums1[i1];
                break;
            }
            if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        common
    }
}

fn main() {
    println!("{} {}", Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
    println!(
        "{} {}",
        Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]),
        2
    );
}
