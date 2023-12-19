struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img.first().unwrap().len();
        let mut smooth_img = vec![];
        for i in 0..m {
            let mut row = vec![];
            for j in 0..n {
                let mut sum = 0;
                sum += img.get(i).unwrap().get(j).unwrap();
                let mut count = 1;
                if i as i32 - 1 >= 0 {
                    sum += img.get(i - 1).unwrap().get(j).unwrap();
                    count += 1;
                    if j as i32 - 1 >= 0 {
                        sum += img.get(i - 1).unwrap().get(j - 1).unwrap();
                        count += 1;
                    }
                    if j + 1 < n {
                        sum += img.get(i - 1).unwrap().get(j + 1).unwrap();
                        count += 1;
                    }
                }
                if i + 1 < m {
                    sum += img.get(i + 1).unwrap().get(j).unwrap();
                    count += 1;
                    if j as i32 - 1 >= 0 {
                        sum += img.get(i + 1).unwrap().get(j - 1).unwrap();
                        count += 1;
                    }
                    if j + 1 < n {
                        sum += img.get(i + 1).unwrap().get(j + 1).unwrap();
                        count += 1;
                    }
                }
                if j as i32 - 1 >= 0 {
                    sum += img.get(i).unwrap().get(j - 1).unwrap();
                    count += 1;
                }
                if j + 1 < n {
                    sum += img.get(i).unwrap().get(j + 1).unwrap();
                    count += 1;
                }
                row.push(sum / count);
            }
            smooth_img.push(row);
        }
        smooth_img
    }
}

fn main() {
    println!(
        "{:?}, {:?}",
        Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
    println!(
        "{:?}, {:?}",
        Solution::image_smoother(vec![
            vec![100, 200, 100],
            vec![200, 50, 200],
            vec![100, 200, 100]
        ]),
        vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137]
        ]
    );
}
