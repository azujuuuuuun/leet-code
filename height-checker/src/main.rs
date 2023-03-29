struct Solution {}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut heights_for_sort = heights.clone();

        heights_for_sort.sort();

        let mut num = 0;
        heights.iter().enumerate().for_each(|(i, h)| {
            if h != &heights_for_sort[i] {
                num += 1
            }
        });

        num
    }
}

fn main() {
    println!("{}", Solution::height_checker(vec![1, 1, 4, 2, 1, 3]));
    println!("{}", Solution::height_checker(vec![5, 1, 2, 3, 4]));
    println!("{}", Solution::height_checker(vec![1, 2, 3, 4, 5]));
}
