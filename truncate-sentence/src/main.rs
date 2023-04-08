struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut ss: Vec<_> = s.split(' ').collect();

        ss.truncate(k as usize);

        ss.join(" ")
    }
}

fn main() {
    println!(
        "{}",
        Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4)
    );
    println!(
        "{}",
        Solution::truncate_sentence("What is the solution to this problem".to_string(), 4)
    );
    println!(
        "{}",
        Solution::truncate_sentence("chopper is not a tanuki".to_string(), 5)
    );
}
