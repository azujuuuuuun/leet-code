struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        tokens.iter().for_each(|s| {
            if s == "+" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 + num2);
            } else if s == "-" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 - num2);
            } else if s == "*" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 * num2);
            } else if s == "/" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 / num2);
            } else {
                stack.push(s.parse().unwrap());
            }
        });
        stack.pop().unwrap()
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::eval_rpn(vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string()
        ]),
        9
    );
    println!(
        "{} {}",
        Solution::eval_rpn(vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string()
        ]),
        6
    );
    println!(
        "{} {}",
        Solution::eval_rpn(vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string()
        ]),
        22
    );
}
