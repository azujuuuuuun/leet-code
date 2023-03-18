use std::cmp::{max, min};

struct BrowserHistory {
    histories: Vec<String>,
    current_index: i32,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            histories: vec![homepage],
            current_index: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.histories = self.histories[0..(self.current_index + 1) as usize].to_vec();
        self.histories.push(url);
        self.current_index = (self.histories.len() - 1) as i32;
    }

    fn back(&mut self, steps: i32) -> String {
        self.current_index = max(self.current_index - steps, 0);
        return self.histories[self.current_index as usize].clone();
    }

    fn forward(&mut self, steps: i32) -> String {
        self.current_index = min(
            self.current_index + steps,
            (self.histories.len() - 1) as i32,
        );
        return self.histories[self.current_index as usize].clone();
    }
}

fn main() {
    let mut obj = BrowserHistory::new(String::from("leetcode.com"));
    obj.visit(String::from("google.com"));
    obj.visit(String::from("facebook.com"));
    obj.visit(String::from("youtube.com"));
    println!("{}", obj.back(1));
    println!("{}", obj.back(1));
    println!("{}", obj.forward(1));
    obj.visit(String::from("linkedin.com"));
    println!("{}", obj.forward(2));
    println!("{}", obj.back(2));
    println!("{}", obj.back(7));
}
