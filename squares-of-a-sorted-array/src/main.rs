struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // let mut squares = vec![-1; nums.len()];
        // nums.iter().for_each(|n| {
        //     let mut square = n * n;
        //     for i in 0..squares.len() {
        //         if squares[i] == -1 {
        //             squares[i] = square;
        //             break;
        //         }
        //         if square < squares[i] {
        //             let tmp = squares[i];
        //             squares[i] = square;
        //             square = tmp;
        //         }
        //     }
        // });
        // squares

        let mut squares = nums.iter().map(|n| n * n).collect::<Vec<_>>();
        squares.sort();
        squares
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    println!(
        "{:?} {:?}",
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
