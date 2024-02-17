use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut bricks_mut = bricks;
        let mut ladders_mut = ladders;
        let mut index = 0;
        for i in 0..(heights.len() - 1) {
            let diff = heights[i + 1] - heights[i];
            if diff <= 0 {
                index = i + 1;
                continue;
            }
            bricks_mut -= diff;
            heap.push(diff);
            if bricks_mut < 0 {
                bricks_mut += heap.pop().unwrap();
                ladders_mut -= 1;
            }
            if ladders_mut < 0 {
                index = i;
                break;
            }
            if i == heights.len() - 2 {
                index = i + 1;
            }
        }
        index as i32
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1),
        4
    );
    println!(
        "{} {}",
        Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
        7
    );
    println!(
        "{} {}",
        Solution::furthest_building(vec![14, 3, 19, 3], 17, 0),
        3
    );
}
