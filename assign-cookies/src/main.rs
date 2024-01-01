struct Solution {}

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut gc = g.clone();
        let mut sc = s.clone();
        gc.sort();
        sc.sort();
        let mut i = 0;
        let mut j = 0;
        loop {
            if i >= gc.len() || j >= sc.len() {
                break;
            }
            if gc.get(i) <= sc.get(j) {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i as i32
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
        1
    );
    println!(
        "{} {}",
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
        2
    );
}
