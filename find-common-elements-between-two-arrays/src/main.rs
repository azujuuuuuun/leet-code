struct Solution {}

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1_count = 0;
        let mut nums2_count = 0;
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    nums1_count += 1;
                    break;
                }
            }
        }
        for i in 0..nums2.len() {
            for j in 0..nums1.len() {
                if nums2[i] == nums1[j] {
                    nums2_count += 1;
                    break;
                }
            }
        }
        vec![nums1_count, nums2_count]
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
        vec![3, 4]
    );
    println!(
        "{:?} {:?}",
        Solution::find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]),
        vec![0, 0]
    );
}
