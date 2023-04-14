struct Solution {}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    height: i32,
}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people = names
            .iter()
            .enumerate()
            .map(|(i, n)| Person {
                name: n.to_string(),
                height: heights[i],
            })
            .collect::<Vec<_>>();

        people.sort_by(|a, b| b.height.cmp(&a.height));

        people.iter().map(|p| p.name.to_string()).collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sort_people(
            vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
            vec![180, 165, 170]
        )
    );
    println!(
        "{:?}",
        Solution::sort_people(
            vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
            vec![155, 185, 150]
        )
    );
}
