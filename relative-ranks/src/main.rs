struct Solution {}

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score_for_sort = score.clone();
        score_for_sort.sort_by(|a, b| b.cmp(a));

        score
            .iter()
            .map(|s| {
                let i = score_for_sort.iter().position(|ss| ss == s).unwrap();
                if i == 0 {
                    return "Gold Medal".to_string();
                } else if i == 1 {
                    return "Silver Medal".to_string();
                } else if i == 2 {
                    return "Bronze Medal".to_string();
                } else {
                    return (i + 1).to_string();
                }
            })
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]));
    println!("{:?}", Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]));
}
