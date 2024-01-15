struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut tmp = num;
        let mut count = 0;
        let mut complement = 0;
        loop {
            if tmp < 1 {
                break;
            }
            if tmp % 2 == 0 {
                complement += 2_i32.pow(count);
            }
            tmp /= 2;
            count += 1;
        }
        complement
    }
}

fn main() {
    println!("{} {}", Solution::find_complement(5), 2);
    println!("{} {}", Solution::find_complement(1), 0);
}
