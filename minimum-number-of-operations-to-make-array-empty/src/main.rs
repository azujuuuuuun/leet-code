use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        nums.iter().for_each(|n| {
            match map.get(n) {
                Some(count) => map.insert(n, count + 1),
                None => map.insert(n, 1),
            };
        });
        let mut operation_nums = 0;
        let mut impossible = false;
        map.into_iter().for_each(|(_, count)| {
            if count == 1 {
                impossible = true;
            } else if count % 3 == 0 {
                operation_nums += count / 3;
            } else if count % 3 == 1 {
                operation_nums += (count / 3 - 1) + (count - (count / 3 - 1) * 3) / 2;
            } else if count % 3 == 2 {
                operation_nums += count / 3 + (count - count / 3 * 3) / 2;
            }
        });
        if impossible {
            -1
        } else {
            operation_nums
        }
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]),
        4
    );
    println!(
        "{} {}",
        Solution::min_operations(vec![2, 1, 2, 2, 3, 3]),
        -1
    );
    println!(
        "{} {}",
        Solution::min_operations(vec![
            14, 12, 14, 14, 12, 14, 14, 12, 12, 12, 12, 14, 14, 12, 14, 14, 14, 12, 12
        ]),
        7
    );
    println!(
        "{} {}",
        Solution::min_operations(vec![1, 1, 1, 1, 1, 1, 1]),
        3
    );
}
