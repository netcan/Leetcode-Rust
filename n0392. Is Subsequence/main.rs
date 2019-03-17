// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let (mut i, mut j) = (0, 0);
        while i < s.len() && j < t.len() {
            if s[i] == t[j] { i += 1; }
            j += 1;
        }
        if i == s.len() { true }
        else { false }
    }
}

