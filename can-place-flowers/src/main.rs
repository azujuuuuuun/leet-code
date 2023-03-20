struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut bed = flowerbed.clone();
        let mut cnt = 0;
        let mut index = 0;

        while index < bed.len() {
            if bed[index] == 1 {
                index += 1;
                continue;
            }

            if index == 0 {
                if index + 1 <= flowerbed.len() - 1 {
                    if bed[index + 1] == 0 {
                        bed[index] = 1;
                        cnt += 1;
                    }
                } else {
                    bed[index] = 1;
                    cnt += 1;
                }
            } else if index == flowerbed.len() - 1 {
                if bed[index - 1] == 0 {
                    bed[index] = 1;
                    cnt += 1;
                }
            } else {
                if bed[index - 1] == 0 && bed[index + 1] == 0 {
                    bed[index] = 1;
                    cnt += 1;
                }
            }

            index += 1;
        }

        if cnt >= n {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    println!("{}", Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    println!("{}", Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2),);
}
