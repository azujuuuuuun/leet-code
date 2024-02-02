struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        vec![
            12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345, 3456,
            4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678, 456789,
            1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
        ]
        .into_iter()
        .filter(|n| *n >= low && *n <= high)
        .collect::<Vec<i32>>()
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::sequential_digits(100, 300),
        vec![123, 234]
    );
    println!(
        "{:?} {:?}",
        Solution::sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}
