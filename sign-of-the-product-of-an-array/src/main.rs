struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut negative_count = 0;

        for n in nums {
            if n == 0 {
                return 0;
            }
            if n < 0 {
                negative_count += 1;
            }
        }

        if negative_count % 2 == 0 {
            1
        } else {
            -1
        }
    }
}

fn main() {
    println!("{}", Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));
    println!("{}", Solution::array_sign(vec![1, 5, 0, 2, -3]));
    println!("{}", Solution::array_sign(vec![-1, 1, -1, 1, -1]));
}
