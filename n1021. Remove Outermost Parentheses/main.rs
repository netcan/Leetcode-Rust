// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let s = s.into_bytes();
        let mut lev = 0;
        let mut primitive = String::new();
        for &p in &s {
            if p == '(' as u8 {
                if lev > 0 { primitive += "("; }
                lev += 1;
            } else { // ')'
                lev -= 1;
                if lev > 0 { primitive += ")"; }
            }
        }

        primitive
    }
}
