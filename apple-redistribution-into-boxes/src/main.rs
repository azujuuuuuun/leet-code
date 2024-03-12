struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut sum = apple.iter().sum::<i32>();
        let mut mut_capacity = capacity.clone();
        mut_capacity.sort();
        mut_capacity.reverse();
        let mut count = 0;
        for i in 0..mut_capacity.len() {
            let c = mut_capacity[i];
            sum -= c;
            count += 1;
            if sum <= 0 {
                break;
            }
        }
        count
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
        2
    );
    println!(
        "{} {}",
        Solution::minimum_boxes(vec![5, 5, 5,], vec![2, 4, 2, 7]),
        4
    );
}
