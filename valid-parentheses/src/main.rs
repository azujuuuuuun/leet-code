struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut valid = true;

        s.chars().for_each(|c| {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            } else if c == ')' {
                let left = stack.pop();
                match left {
                    Some(p) if p != '(' => valid = false,
                    None => valid = false,
                    _ => {}
                }
            } else if c == '}' {
                let left = stack.pop();
                match left {
                    Some(p) if p != '{' => valid = false,
                    None => valid = false,
                    _ => {}
                }
            } else if c == ']' {
                let left = stack.pop();
                match left {
                    Some(p) if p != '[' => valid = false,
                    None => valid = false,
                    _ => {}
                }
            }
        });

        if !stack.is_empty() {
            return false;
        }

        valid
    }
}

fn main() {
    println!("{}", Solution::is_valid("()".to_string()));
    println!("{}", Solution::is_valid("()[]{}".to_string()));
    println!("{}", Solution::is_valid("(]".to_string()));
    println!("{}", Solution::is_valid("(())".to_string()));
}
