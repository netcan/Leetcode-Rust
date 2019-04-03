// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for c in letters.iter() {
            if *c > target {
                return *c;
            }
        }
        letters[0]
    }
}

