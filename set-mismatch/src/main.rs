use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 1..nums.len() + 1 {
            map.insert(i as i32, 0);
        }
        nums.iter().for_each(|n| {
            if let Some(count) = map.get(n) {
                map.insert(*n, count + 1);
            }
        });
        let mut missing = 0;
        let mut duplicate = 0;
        map.iter().for_each(|(num, count)| {
            if *count == 0 {
                missing = *num;
            } else if *count == 2 {
                duplicate = *num;
            }
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
