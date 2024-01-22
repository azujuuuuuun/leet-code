use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        for i in 1..nums.len() + 1 {
            set.insert(i as i32);
        }
        let mut duplicate = 0;
        let mut missing = 0;
        nums.iter().for_each(|n| {
            if let Some(_) = set.get(n) {
                set.remove(n);
            } else {
                duplicate = *n;
            }
        });
        set.iter().for_each(|n| {
            missing = *n;
        });
        vec![duplicate, missing]
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_error_nums(vec![1, 2, 2, 4]),
        vec![2, 3]
    );
    println!(
        "{:?} {:?}",
        Solution::find_error_nums(vec![1, 1]),
        vec![1, 2]
    );
}
