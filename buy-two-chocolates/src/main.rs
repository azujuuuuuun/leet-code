struct Solution {}

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut min = 101;
        let mut min2 = 101;
        prices.iter().for_each(|p| {
            if p <= &min {
                min2 = min;
                min = p.to_owned();
            } else if p <= &min2 {
                min2 = p.to_owned();
            }
        });
        if min + min2 > money {
            money
        } else {
            money - min - min2
        }
    }
}

fn main() {
    println!("{}, {}", Solution::buy_choco(vec![1, 2, 2], 3), 0);
    println!("{}, {}", Solution::buy_choco(vec![3, 2, 3], 3), 3);
}
