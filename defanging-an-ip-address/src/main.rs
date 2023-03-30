struct Solution {}

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.split('.').collect::<Vec<_>>().join("[.]")
    }
}

fn main() {
    println!("{}", Solution::defang_i_paddr("1.1.1.1".to_string()));
    println!("{}", Solution::defang_i_paddr("255.100.50.0".to_string()));
}
