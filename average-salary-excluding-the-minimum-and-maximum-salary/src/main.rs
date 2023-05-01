use std::i32::{MAX, MIN};

struct Solution {}

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut sum = 0;
        let mut min = MAX;
        let mut max = MIN;

        salary.iter().for_each(|s| {
            sum += s;
            if *s < min {
                min = *s;
            }
            if *s > max {
                max = *s;
            }
        });

        sum -= min + max;

        f64::from(sum) / f64::from((salary.len() - 2) as i32)
    }
}

fn main() {
    println!("{}", Solution::average(vec![4000, 3000, 1000, 2000]));
    println!("{}", Solution::average(vec![1000, 2000, 3000]));
}
