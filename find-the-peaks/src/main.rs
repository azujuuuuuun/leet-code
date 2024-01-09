struct Solution {}

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut peeks = vec![];
        mountain.iter().enumerate().for_each(|(i, m)| {
            if i != 0 && i != mountain.len() - 1 && *m > mountain[i - 1] && *m > mountain[i + 1] {
                peeks.push(i as i32);
            }
        });
        peeks
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::find_peaks(vec![2, 4, 4]),
        vec![] as Vec<i32>
    );
    println!(
        "{:?} {:?}",
        Solution::find_peaks(vec![1, 4, 3, 8, 5]),
        vec![1, 3]
    );
}
