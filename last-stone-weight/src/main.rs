struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.len() == 1 {
            return stones[0];
        }

        let mut mutable_stones = stones;

        mutable_stones.sort();

        while mutable_stones.len() > 1 {
            let y = mutable_stones.pop().unwrap();
            let x = mutable_stones.pop().unwrap();

            if x == y {
                continue;
            }

            mutable_stones.push(y - x);
            mutable_stones.sort();
        }

        if mutable_stones.is_empty() {
            0
        } else {
            mutable_stones[0]
        }
    }
}

fn main() {
    println!("{}", Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    println!("{}", Solution::last_stone_weight(vec![1]));
}
