struct Solution {}

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = vec![];

        s.chars().for_each(|c| {
            if c != '*' {
                stack.push(c);
            } else {
                stack.pop();
            }
        });

        stack
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

fn main() {
    println!("{}", Solution::remove_stars("leet**cod*e".to_string()));
    println!("{}", Solution::remove_stars("erase*****".to_string()));
}
