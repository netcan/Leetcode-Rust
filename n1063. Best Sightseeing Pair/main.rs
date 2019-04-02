// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let (mut max_score, mut max_pair) = (0, 0);
        for i in 0..a.len() {
            max_score = (a[i] - i as i32 + max_pair).max(max_score);
            max_pair = max_pair.max(a[i] + i as i32);
        }

        max_score
    }
}

