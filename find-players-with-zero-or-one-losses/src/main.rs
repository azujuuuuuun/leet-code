use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
        matches.iter().for_each(|m| {
            let winner = m[0];
            let loser = m[1];
            if let Some(result) = map.get(&winner) {
                map.insert(winner, (result.0 + 1, result.1));
            } else {
                map.insert(winner, (1, 0));
            }
            if let Some(result) = map.get(&loser) {
                map.insert(loser, (result.0, result.1 + 1));
            } else {
                map.insert(loser, (0, 1));
            }
        });

        let mut winners = vec![];
        let mut losers = vec![];
        map.iter().for_each(|(player, (_, lose))| {
            if *lose == 0 {
                winners.push(*player);
            } else if *lose == 1 {
                losers.push(*player);
            }
        });
        winners.sort();
        losers.sort();

        vec![winners, losers]
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_winners(vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9]
        ]),
        vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
    );
    println!(
        "{:?} {:?}",
        Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4],]),
        vec![vec![1, 2, 5, 6], vec![]]
    );
}
