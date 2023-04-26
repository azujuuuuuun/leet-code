struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut digit = num;

        loop {
            if digit / 10 < 1 {
                break;
            }

            let mut tmp = digit;
            let mut sum = 0;
            loop {
                if tmp / 10 < 1 {
                    sum += tmp;
                    break;
                }

                sum += tmp % 10;
                tmp /= 10;
            }

            digit = sum;
        }

        digit
    }
}

fn main() {
    println!("{}, {}", Solution::add_digits(38), 2);
    println!("{}, {}", Solution::add_digits(0), 0);
    println!("{}, {}", Solution::add_digits(1), 1);
    println!("{}, {}", Solution::add_digits(10), 1);
    println!("{}, {}", Solution::add_digits(11), 2);
}
