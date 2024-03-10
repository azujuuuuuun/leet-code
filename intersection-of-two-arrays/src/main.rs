use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        nums1.iter().for_each(|n| {
            set1.insert(*n);
        });
        nums2.iter().for_each(|n| {
            set2.insert(*n);
        });
        let mut ans = vec![];
        set1.iter().for_each(|n| {
            if set2.contains(n) {
                ans.push(*n);
            }
        });
        ans
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    );
    println!(
        "{:?} {:?}",
        Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        vec![9, 4]
    );
}
