struct Solution {}

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let first = nums[0];
        let second = nums[1];
        let third = nums[2];
        if first == second && second == third {
            "equilateral".to_string()
        } else if (first + second <= third)
            || (second + third <= first)
            || (third + first <= second)
        {
            "none".to_string()
        } else if first != second && second != third && third != first {
            "scalene".to_string()
        } else {
            "isosceles".to_string()
        }
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::triangle_type(vec![3, 3, 3]),
        "equilateral".to_string()
    );
    println!(
        "{} {}",
        Solution::triangle_type(vec![3, 4, 5]),
        "scalene".to_string()
    );
    println!(
        "{} {}",
        Solution::triangle_type(vec![9, 4, 9]),
        "isosceles".to_string()
    );
}
