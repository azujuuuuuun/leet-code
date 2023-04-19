struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut num = 0;

        logs.iter().for_each(|o| {
            if o == "../" {
                num = if num == 0 { 0 } else { num - 1 };
            } else if o == "./" {
                return;
            } else {
                num += 1;
            }
        });

        num
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string()
        ])
    );
    println!(
        "{}",
        Solution::min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31".to_string()
        ])
    );
    println!(
        "{}",
        Solution::min_operations(vec![
            "d1/".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string(),
        ])
    );
}
