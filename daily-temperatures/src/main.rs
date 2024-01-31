use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut dequeue = VecDeque::new();
        let mut answer = vec![0; temperatures.len()];
        for i in (0..temperatures.len()).rev() {
            if dequeue.is_empty() {
                dequeue.push_front(i);
                answer[i] = 0;
            } else {
                loop {
                    if dequeue.is_empty()
                        || temperatures[i] < temperatures[*dequeue.front().unwrap()]
                    {
                        break;
                    }
                    dequeue.pop_front();
                }
                if dequeue.is_empty() {
                    answer[i] = 0;
                } else {
                    answer[i] = (dequeue.front().unwrap() - i) as i32;
                }
                dequeue.push_front(i);
            }
        }
        answer
    }
}

fn main() {
    println!(
        "{:?} {:?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    println!(
        "{:?} {:?}",
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    );
    println!(
        "{:?} {:?}",
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    );
    println!(
        "{:?} {:?}",
        Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
        vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
    );
}
