// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        Solution::score_of_parentheses_(&s)
    }

    fn score_of_parentheses_(s: &str) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut level = 0;
        let mut score = 0;
        while right < s.len() {
            if s.as_bytes()[right] == '(' as u8 {
                level += 1;
                if level == 1 { left = right; }
            } else {
                level -= 1;
                if level == 0 { // 找到一对括号
                    if left + 1 < right { // 嵌套 * 2
                        score += Solution::score_of_parentheses_(&s[left + 1 .. right]) * 2;
                    } else {
                        score += 1;
                    }
                }
            }
            right += 1;
        }
        score
    }
}

