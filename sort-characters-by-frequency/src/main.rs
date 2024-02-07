use std::collections::HashMap;

#[derive(Debug)]
struct CharCount {
    c: char,
    count: i32,
}

struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        s.chars().into_iter().for_each(|c| {
            if let Some(count) = map.get(&c) {
                map.insert(c, *count + 1);
            } else {
                map.insert(c, 1);
            }
        });

        let mut char_counts = map
            .iter()
            .map(|(c, count)| CharCount {
                c: *c,
                count: *count,
            })
            .collect::<Vec<_>>();
        char_counts.sort_by(|a, b| b.count.cmp(&a.count).then(a.c.cmp(&b.c)));

        char_counts
            .iter()
            .map(|cc| cc.c.to_string().repeat(cc.count as usize))
            .reduce(|acc, e| acc + &e)
            .unwrap()
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::frequency_sort("tree".to_string()),
        "eert"
    );
    println!(
        "{} {}",
        Solution::frequency_sort("cccaaa".to_string()),
        "aaaccc"
    );
    println!(
        "{} {}",
        Solution::frequency_sort("Aabb".to_string()),
        "bbAa"
    );
}
