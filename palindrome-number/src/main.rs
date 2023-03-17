impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }

        let mut digits = vec![];
        let mut y = x.clone();
        loop {
            if y < 1 {
                break;
            }
            digits.push(y % 10);
            y /= 10;
        }

        let mut palindrome = true;

        for (i, _) in digits.iter().enumerate() {
            if digits[i] != digits[digits.len() - 1 - i] {
                palindrome = false;
            }
        }

        return palindrome;
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::is_palindrome(121));
    println!("{:?}", Solution::is_palindrome(-121));
    println!("{:?}", Solution::is_palindrome(10));
}
