use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        Self::dfs(&arr, 0, &HashSet::new(), 0)
    }

    fn dfs(arr: &Vec<String>, index: usize, set: &HashSet<char>, length: i32) -> i32 {
        if index >= arr.len() {
            return length;
        }

        let len1 = Self::dfs(arr, index + 1, set, length);

        let mut len2 = 0;
        let mut cloned_set = set.clone();
        arr[index].chars().for_each(|c| {
            cloned_set.insert(c);
        });
        if cloned_set.len() as i32 == length + arr[index].len() as i32 {
            len2 = Self::dfs(arr, index + 1, &cloned_set, cloned_set.len() as i32);
        }

        if len1 > len2 {
            len1
        } else {
            len2
        }
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]),
        4
    );
    println!(
        "{} {}",
        Solution::max_length(vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string()
        ]),
        6
    );
    println!(
        "{} {}",
        Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string(),]),
        26
    );
}
