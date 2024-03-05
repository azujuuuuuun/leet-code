struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut li = 0;
        let mut ri = s.len() - 1;
        let mut lc = chars[li];
        let mut rc = chars[ri];
        let mut ans = (ri - li + 1) as i32;
        loop {
            if lc != rc {
                break;
            }
            loop {
                if chars[li + 1] == lc {
                    li += 1;
                    if li == ri {
                        break;
                    }
                } else {
                    break;
                }
            }
            if li == ri {
                ans = 0;
                break;
            }
            loop {
                if chars[ri - 1] == rc {
                    ri -= 1;
                } else {
                    break;
                }
            }
            if ri - li == 2 {
                ans = 1;
                break;
            }
            ans = ri as i32 - li as i32 - 1;
            li += 1;
            lc = chars[li];
            ri -= 1;
            rc = chars[ri];
        }
        ans
    }
}

fn main() {
    println!("{} {}", Solution::minimum_length("ca".to_string()), 2);
    println!("{} {}", Solution::minimum_length("cabaabac".to_string()), 0);
    println!(
        "{} {}",
        Solution::minimum_length("aabccabba".to_string()),
        3
    );
    println!("{} {}", Solution::minimum_length("abcba".to_string()), 1);
    println!("{} {}", Solution::minimum_length("c".to_string()), 1);
}
