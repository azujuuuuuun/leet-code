struct Solution {}

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let len = s.len();
        let chars: Vec<_> = s.chars().collect();
        let mut count = 0;
        let mut stars_removed_chars = vec![];

        for i in 0..len {
            let c = chars[len - 1 - i];
            if c == '*' {
                count += 1;
            } else if count == 0 {
                stars_removed_chars.push(c);
            } else {
                count -= 1;
            }
        }

        stars_removed_chars.reverse();

        stars_removed_chars
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
