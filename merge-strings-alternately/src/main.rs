struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut i1 = 0;
        let mut i2 = 0;
        let mut merged_string = "".to_string();

        while i1 != word1.len() && i2 != word2.len() {
            merged_string += word1.get(i1..i1 + 1).unwrap();
            i1 += 1;
            merged_string += word2.get(i2..i2 + 1).unwrap();
            i2 += 1;
        }

        if i1 != word1.len() {
            merged_string += word1.get(i1..).unwrap();
        }
        if i2 != word2.len() {
            merged_string += word2.get(i2..).unwrap();
        }

        merged_string
    }
}

fn main() {
    println!(
        "{}",
        Solution::merge_alternately("abc".to_string(), "pqr".to_string())
    );
    println!(
        "{}",
        Solution::merge_alternately("ab".to_string(), "pqrs".to_string())
    );
    println!(
        "{}",
        Solution::merge_alternately("abcd".to_string(), "pq".to_string())
    );
}
