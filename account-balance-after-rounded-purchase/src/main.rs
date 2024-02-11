struct Solution {}

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (((purchase_amount as f64 / 10 as f64).round() * (10 as f64)) as i32)
    }
}

fn main() {
    println!("{} {}", Solution::account_balance_after_purchase(9), 90);
    println!("{} {}", Solution::account_balance_after_purchase(15), 80);
}
