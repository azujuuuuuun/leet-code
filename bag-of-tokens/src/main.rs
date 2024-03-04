struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }

        let mut mut_tokens = tokens.clone();
        let mut mut_power = power;
        let mut score = 0;
        let mut max_score = 0;
        let mut left_index = 0;
        let mut right_index = tokens.len() - 1;

        mut_tokens.sort();

        loop {
            if left_index > right_index {
                break;
            }
            if mut_power < mut_tokens[left_index] && score == 0 {
                break;
            }
            loop {
                if left_index >= mut_tokens.len() {
                    break;
                }
                if mut_power < mut_tokens[left_index] {
                    break;
                }
                mut_power -= mut_tokens[left_index];
                score += 1;
                left_index += 1;
                if score > max_score {
                    max_score = score;
                }
            }
            if left_index >= mut_tokens.len() {
                break;
            }
            if score > 0 {
                mut_power += mut_tokens[right_index];
                score -= 1;
                right_index -= 1;
            }
        }

        max_score
    }
}

fn main() {
    println!("{} {}", Solution::bag_of_tokens_score(vec![100], 50), 0);
    println!(
        "{} {}",
        Solution::bag_of_tokens_score(vec![200, 100], 150),
        1
    );
    println!(
        "{} {}",
        Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
        2
    );
    println!("{} {}", Solution::bag_of_tokens_score(vec![], 85), 0);
    println!("{} {}", Solution::bag_of_tokens_score(vec![26], 51), 1);
}
