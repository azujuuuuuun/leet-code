struct Solution {}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if Solution::is_prefix_and_suffix(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_prefix_and_suffix(str1: &String, str2: &String) -> bool {
        str2.starts_with(str1) && str2.ends_with(str1)
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::count_prefix_suffix_pairs(vec![
            "a".to_string(),
            "aba".to_string(),
            "ababa".to_string(),
            "aa".to_string()
        ]),
        4
    );
    println!(
        "{} {}",
        Solution::count_prefix_suffix_pairs(vec![
            "pa".to_string(),
            "papa".to_string(),
            "ma".to_string(),
            "mama".to_string()
        ]),
        2
    );
    println!(
        "{} {}",
        Solution::count_prefix_suffix_pairs(vec!["abab".to_string(), "ab".to_string(),]),
        0
    );
}
