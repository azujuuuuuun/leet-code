struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut cost = 0;
        let mut i = 0;
        let mut j = 0;
        loop {
            if i >= colors.len() || j >= colors.len() {
                break;
            }
            let mut sum = 0;
            let mut max = 0;
            loop {
                if j >= colors.len()
                    || colors.get(i..i + 1).unwrap() != colors.get(j..j + 1).unwrap()
                {
                    break;
                }
                let time = needed_time.get(j).unwrap().to_owned();
                sum += time;
                if time >= max {
                    max = time;
                }
                j += 1;
            }
            cost += sum - max;
            i = j;
        }
        cost
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
        3
    );
    println!(
        "{} {}",
        Solution::min_cost("abc".to_string(), vec![1, 2, 3]),
        0
    );
    println!(
        "{} {}",
        Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
        2
    );
    println!(
        "{} {}",
        Solution::min_cost("aaaaa".to_string(), vec![1, 2, 3, 4, 1]),
        7
    );
}
