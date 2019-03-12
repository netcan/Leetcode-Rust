// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() == 0 {
            return 0;
        }
        return words[words.len() - 1].len() as i32;

    }
}

