struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trusts = vec![(0, 0); (n + 1) as usize];
        trust.iter().for_each(|t| {
            let a = t[0];
            let b = t[1];
            trusts[a as usize] = (trusts[a as usize].0, trusts[a as usize].1 + 1);
            trusts[b as usize] = (trusts[b as usize].0 + 1, trusts[b as usize].1);
        });
        let mut town_judge = -1;
        for i in 1..n + 1 {
            if trusts[i as usize].0 == n - 1 && trusts[i as usize].1 == 0 {
                town_judge = i;
            }
        }
        town_judge
    }
}

fn main() {
    println!("{} {}", Solution::find_judge(2, vec![vec![1, 2]]), 2);
    println!(
        "{} {}",
        Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]),
        3
    );
    println!(
        "{} {}",
        Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
        -1
    );
    println!(
        "{} {}",
        Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]),
        -1
    );
}
