// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        // 最多有s_len ^ 2种子串
        if s.len() * s.len() < n as usize { return false; }
        for i in 1..=n {
            if let None = s.find(&format!("{:b}", i)) {
                return false;
            }
        }
        true
    }
}

