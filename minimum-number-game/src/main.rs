struct Solution {}

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut cloned_nums = nums.clone();
        cloned_nums.sort();
        let mut arr = vec![];
        for i in 0..cloned_nums.len() {
            if i % 2 == 1 {
                continue;
            }
            let num1 = cloned_nums.get(i).unwrap();
            let num2 = cloned_nums.get(i + 1).unwrap();
            arr.push(num2.to_owned());
            arr.push(num1.to_owned());
        }
        arr
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::number_game(vec![5, 4, 2, 3]),
        vec![3, 2, 5, 4]
    );
    println!("{:?} {:?}", Solution::number_game(vec![2, 5]), vec![5, 2]);
}
