struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut answer = vec![];
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if (i as i32 - j as i32).abs() >= index_difference
                    && (nums[i] - nums[j]).abs() >= value_difference
                {
                    answer.push(i as i32);
                    answer.push(j as i32);
                    break;
                }
            }
            if answer.len() == 2 {
                break;
            }
        }
        if answer.len() != 2 {
            answer.push(-1);
            answer.push(-1);
        }
        answer
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_indices(vec![5, 1, 4, 1], 2, 4),
        vec![0, 3]
    );
    println!(
        "{:?} {:?}",
        Solution::find_indices(vec![2, 1], 0, 0),
        vec![0, 0]
    );
    println!(
        "{:?} {:?}",
        Solution::find_indices(vec![1, 2, 3], 2, 4),
        vec![-1, -1]
    );
}
