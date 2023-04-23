struct Solution {}

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let words = text
            .split(' ')
            .filter(|w| !(*w).is_empty())
            .collect::<Vec<_>>();
        let space_num = text.chars().filter(|c| *c == ' ').count();

        if words.len() == 1 {
            return words[0].to_string() + &" ".repeat(space_num);
        }

        words.join(&" ".repeat(space_num / (words.len() - 1)))
            + &" ".repeat(space_num % (words.len() - 1))
    }
}

fn main() {
    println!(
        "{}",
        Solution::reorder_spaces("  this   is  a sentence ".to_string())
    );
    println!(
        "{}",
        Solution::reorder_spaces(" practice   makes   perfect".to_string())
    );
    println!("{}", Solution::reorder_spaces("a".to_string()));
}
