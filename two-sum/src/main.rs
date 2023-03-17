impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                if n1 + n2 == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}
