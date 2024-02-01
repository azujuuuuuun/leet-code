struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut cloned_nums = nums.clone();
        cloned_nums.sort();

        let mut arrays = vec![];
        let mut impossible = false;
        let mut i = 0;
        loop {
            if i >= cloned_nums.len() || impossible {
                break;
            }
            if cloned_nums[i + 2] - cloned_nums[i] <= k {
                arrays.push(vec![cloned_nums[i], cloned_nums[i + 1], cloned_nums[i + 2]]);
            } else {
                impossible = true;
            }
            i += 3;
        }

        if impossible {
            vec![]
        } else {
            arrays
        }
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
        vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
    );
    println!(
        "{:?} {:?}",
        Solution::divide_array(vec![1, 3, 3, 2, 7, 3], 3),
        vec![] as Vec<Vec<i32>>
    );
    println!(
        "{:?} {:?}",
        Solution::divide_array(
            vec![15, 13, 12, 13, 12, 14, 12, 2, 3, 13, 12, 14, 14, 13, 5, 12, 12, 2, 13, 2, 2],
            2
        ),
        vec![] as Vec<Vec<i32>>
    );
}
