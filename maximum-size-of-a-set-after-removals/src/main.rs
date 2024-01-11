use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut nums1_set = HashSet::new();
        let mut nums2_set = HashSet::new();
        nums1.iter().for_each(|n| {
            set.insert(*n);
            nums1_set.insert(*n);
        });
        nums2.iter().for_each(|n| {
            set.insert(*n);
            nums2_set.insert(*n);
        });

        let mut only_nums1_count = 0;
        let mut only_nums2_count = 0;
        let mut both_count = 0;
        set.iter().for_each(|n| {
            let is_in_nums1 = nums1_set.get(n).is_some();
            let is_in_nums2 = nums2_set.get(n).is_some();
            if is_in_nums1 && is_in_nums2 {
                both_count += 1;
            } else if is_in_nums1 && !is_in_nums2 {
                only_nums1_count += 1;
            } else if !is_in_nums1 && is_in_nums2 {
                only_nums2_count += 1;
            }
        });

        let n = (nums1.len() / 2) as i32;
        let mut max_size = 0;
        let mut rest = 0;
        if only_nums1_count > n {
            max_size += n;
        } else {
            max_size += only_nums1_count;
            rest += n - only_nums1_count;
        }
        if only_nums2_count > n {
            max_size += n;
        } else {
            max_size += only_nums2_count;
            rest += n - only_nums2_count;
        }
        max_size += if both_count < rest { both_count } else { rest };

        max_size
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1]),
        2
    );
    println!(
        "{} {}",
        Solution::maximum_set_size(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]),
        5
    );
    println!(
        "{} {}",
        Solution::maximum_set_size(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]),
        6
    );
    println!(
        "{} {}",
        Solution::maximum_set_size(vec![1, 1, 2, 2], vec![1, 1, 3, 3]),
        3
    );
    println!(
        "{} {}",
        Solution::maximum_set_size(vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
        4
    );
}
