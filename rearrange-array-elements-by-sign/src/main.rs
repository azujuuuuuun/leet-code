struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let positive = nums.iter().filter(|n| **n > 0).collect::<Vec<_>>();
        let negative = nums.iter().filter(|n| **n < 0).collect::<Vec<_>>();
        let mut array = vec![];
        for i in 0..nums.len() {
            if i % 2 == 0 {
                array.push(*positive[i / 2]);
            } else {
                array.push(*negative[i / 2]);
            }
        }
        array
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
        vec![3, -2, 1, -5, 2, -4]
    );
    println!(
        "{:?} {:?}",
        Solution::rearrange_array(vec![-1, 1]),
        vec![1, -1]
    );
}
