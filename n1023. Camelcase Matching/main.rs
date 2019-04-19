// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.into_bytes();
        let mut ans = vec![false; queries.len()];
        let queries: Vec<Vec<u8>> = queries.into_iter().map(|x| x.into_bytes()).collect();
        for i in 0..queries.len() {
            let word = &queries[i];
            let (mut j, mut k) = (0, 0);
            while j < word.len() && k < pattern.len() {
                if word[j] == pattern[k] { j += 1; k += 1; }
                else if word[j] >= 'a' as u8 && word[j] <= 'z' as u8 { j += 1; }
                else { break; }
            }
            while j < word.len() {
                if word[j] >= 'a' as u8 && word[j] <= 'z' as u8 { j += 1; }
                else { break; }
            }
            ans[i] = j >= word.len() && k >= pattern.len();
        }

        ans
    }
}
