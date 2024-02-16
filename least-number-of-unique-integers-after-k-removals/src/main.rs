use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        arr.iter().for_each(|n| {
            if let Some(count) = map.get(n) {
                map.insert(*n, *count + 1);
            } else {
                map.insert(*n, 1);
            }
        });

        if k == 0 {
            return map.len() as i32;
        }

        let mut num_counts = map.iter().collect::<Vec<_>>();
        num_counts.sort_by(|a, b| a.1.cmp(b.1));

        let mut ans = 0;
        let mut sum = 0;
        for i in 0..num_counts.len() {
            let num_count = num_counts[i];
            sum += num_count.1;
            if sum == k {
                ans = (num_counts.len() - i - 1) as i32;
                break;
            } else if sum > k {
                ans = (num_counts.len() - i) as i32;
                break;
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1),
        1
    );
    println!(
        "{} {}",
        Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
        2
    );
}
