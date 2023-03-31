struct Solution {}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut tmp: Vec<i32> = vec![];

        arr.iter().for_each(|n| {
            tmp.push(*n);
            if *n == 0 {
                tmp.push(0);
            }
        });

        for i in 0..arr.len() {
            arr[i] = tmp[i];
        }
    }
}

fn main() {
    Solution::duplicate_zeros(&mut vec![1, 0, 2, 3, 0, 4, 5, 0]);
}
