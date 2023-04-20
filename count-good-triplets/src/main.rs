struct Solution {}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;

        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                for k in j + 1..arr.len() {
                    if (arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[k] - arr[i]).abs() <= c
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
    );
    println!(
        "{}",
        Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1)
    );
}
