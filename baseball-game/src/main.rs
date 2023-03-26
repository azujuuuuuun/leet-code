struct Solution {}

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut num_stack: Vec<i32> = vec![];

        operations.iter().for_each(|o| {
            if o == "+" {
                let previous_score2 = num_stack[num_stack.len() - 2];
                let previous_score = num_stack[num_stack.len() - 1];
                num_stack.push(previous_score2 + previous_score);
            } else if o == "D" {
                let previous_score = num_stack[num_stack.len() - 1];
                num_stack.push(previous_score * 2);
            } else if o == "C" {
                num_stack.pop();
            } else {
                num_stack.push(o.parse().unwrap());
            }
        });

        num_stack.iter().sum()
    }
}

fn main() {
    println!(
        "{}",
        Solution::cal_points(
            vec!["5", "2", "C", "D", "+"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        )
    );
    println!(
        "{}",
        Solution::cal_points(
            vec!["5", "-2", "4", "C", "D", "9", "+", "+"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        )
    );
    println!(
        "{}",
        Solution::cal_points(vec!["1", "C"].iter().map(|s| s.to_string()).collect())
    );
}
