use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        nums.iter().for_each(|_| {
            let mut vec = vec![];
            nums.iter().for_each(|_| {
                vec.push(0);
            });
            dp.push(vec);
        });

        let mut map: HashMap<i64, Vec<usize>> = HashMap::new();
        nums.iter().enumerate().for_each(|(i, n)| {
            match map.get_mut(&(*n as i64)) {
                Some(indices) => {
                    indices.push(i);
                }
                None => {
                    map.insert(*n as i64, vec![i]);
                }
            };
        });

        let mut sum = 0;
        for i in 1..nums.len() {
            let mut j = i + 1;
            loop {
                if j >= nums.len() {
                    break;
                }
                let a = 2 * (*nums.get(i).unwrap() as i64) - *nums.get(j).unwrap() as i64;
                if let Some(indices) = map.get(&a) {
                    indices.iter().for_each(|k| {
                        if *k < i {
                            dp[i][j] += dp[*k][i] + 1;
                        }
                    });
                }
                sum += dp[i][j];

                j += 1;
            }
        }

        sum
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
        7
    );
    println!(
        "{} {}",
        Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
        16
    );
    println!(
        "{} {}",
        Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
        0
    );
}
