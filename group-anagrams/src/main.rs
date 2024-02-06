struct Solution {}

#[derive(Debug)]
struct Tmp {
    original: String,
    sorted: String,
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut tmps = strs
            .iter()
            .map(|s| {
                let original = s.clone();
                let mut sorted = original.chars().collect::<Vec<_>>();
                sorted.sort();
                Tmp {
                    original,
                    sorted: sorted.iter().collect(),
                }
            })
            .collect::<Vec<Tmp>>();

        tmps.sort_by(|a, b| a.sorted.cmp(&b.sorted));

        let mut groups = vec![];
        let first = tmps.first().unwrap();
        let mut current = first.sorted.to_owned();
        groups.push(vec![first.original.to_owned()]);
        for i in 1..tmps.len() {
            if tmps[i].sorted == current {
                let row = groups.len() - 1;
                groups[row].push(tmps[i].original.to_owned());
            } else {
                groups.push(vec![tmps[i].original.to_owned()]);
            }
            current = tmps[i].sorted.to_owned();
        }

        groups
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ]),
        vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
        ]
    );
    println!(
        "{:?} {:?}",
        Solution::group_anagrams(vec!["".to_string(),]),
        vec![vec!["".to_string()],]
    );
    println!(
        "{:?} {:?}",
        Solution::group_anagrams(vec!["a".to_string(),]),
        vec![vec!["a".to_string()],]
    );
}
