// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        let mut title = String::new();
        while n != 0 {
            title = ((((n - 1) % 26) as u8 + 'A' as u8) as char).to_string() + &title;
            n = (n - 1) / 26;
        }
        title
    }
}

