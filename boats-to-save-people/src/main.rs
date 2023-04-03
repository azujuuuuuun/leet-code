struct Solution {}

impl Solution {
    fn lower_bound(vec: &Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = vec.len();

        while right - left > 1 {
            let middle = left + (right - left) / 2;
            if vec[middle] >= target {
                right = middle;
            } else {
                left = middle;
            }
        }

        left
    }

    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people_clone = people.clone();
        people_clone.sort();

        let mut nums = 0;
        while !people_clone.is_empty() {
            let p = people_clone.pop().unwrap();
            if people_clone.is_empty() {
                nums += 1;
                break;
            }

            let rest = limit - p;
            if rest == 0 {
                nums += 1;
                continue;
            }

            let index = Solution::lower_bound(&people_clone, rest);
            if people_clone[index] + p <= limit {
                people_clone.remove(index);
                nums += 1;
                continue;
            }
            nums += 1;
        }

        nums
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![1, 2], 3));
    println!("{}", Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
    println!("{}", Solution::num_rescue_boats(vec![3, 5, 3, 4], 5));
}
