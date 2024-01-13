use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1_map = HashMap::new();
        let mut nums2_map = HashMap::new();
        nums1.iter().for_each(|n| {
            if let Some(count) = nums1_map.get(n) {
                nums1_map.insert(*n, count + 1);
            } else {
                nums1_map.insert(*n, 1);
            }
        });
        nums2.iter().for_each(|n| {
            if let Some(count) = nums2_map.get(n) {
                nums2_map.insert(*n, count + 1);
            } else {
                nums2_map.insert(*n, 1);
            }
        });

        let mut nums1_count = 0;
        let mut nums2_count = 0;
        nums1_map.iter().for_each(|(n, c)| {
            if let Some(_) = nums2_map.get(n) {
                nums1_count += c;
            }
        });
        nums2_map.iter().for_each(|(n, c)| {
            if let Some(_) = nums1_map.get(n) {
                nums2_count += c;
            }
        });
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
