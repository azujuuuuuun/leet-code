use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (a, b) = s.split_at(s.len() / 2);

        let mut vowels = HashSet::new();
        vowels.insert('a');
        vowels.insert('e');
        vowels.insert('i');
        vowels.insert('o');
        vowels.insert('u');
        vowels.insert('A');
        vowels.insert('E');
        vowels.insert('I');
        vowels.insert('O');
        vowels.insert('U');

        let mut a_vowel_count = 0;
        let mut b_vowel_count = 0;
        a.chars().for_each(|c| {
            if vowels.get(&c).is_some() {
                a_vowel_count += 1;
            };
        });
        b.chars().for_each(|c| {
            if vowels.get(&c).is_some() {
                b_vowel_count += 1;
            };
        });

        a_vowel_count == b_vowel_count
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::halves_are_alike("book".to_string()),
        true
    );
    println!(
        "{} {}",
        Solution::halves_are_alike("textbook".to_string()),
        false
    );
    println!(
        "{} {}",
        Solution::halves_are_alike("bOok".to_string()),
        true
    );
}
