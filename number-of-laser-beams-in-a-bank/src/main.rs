struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut beam_count = 0;
        let mut r1_device_count = 0;
        let mut r2_device_count = 0;
        bank.iter().for_each(|row| {
            row.chars().into_iter().for_each(|c| {
                if c == '1' {
                    r2_device_count += 1;
                }
            });
            if r2_device_count != 0 || r1_device_count == 0 {
                beam_count += r1_device_count * r2_device_count;
                r1_device_count = r2_device_count;
                r2_device_count = 0;
            }
        });
        beam_count
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::number_of_beams(vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ]),
        8
    );
    println!(
        "{} {}",
        Solution::number_of_beams(vec![
            "000".to_string(),
            "111".to_string(),
            "000".to_string(),
        ]),
        0
    );
}
