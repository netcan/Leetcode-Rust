// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // 转换成LCS问题
        let (m, n) = (word1.len() as i32, word2.len() as i32);
        let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
        let mut dp:Vec<Vec<i32>> = iter::repeat(vec![0; word2.len() + 1]).take(word1.len() + 1).collect();
        for (i, &c1) in word1.iter().enumerate() {
            for (j, &c2) in word2.iter().enumerate() {
                dp[i + 1][j + 1] = 
                    if c1 == c2 { dp[i][j] + 1 }
                    else { dp[i][j + 1].max(dp[i+1][j]) }
            }
        }

        m + n - 2 * dp[m as usize][n as usize]
    }
}

