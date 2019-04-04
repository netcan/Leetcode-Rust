## Delete Operation for Two Strings :star::star:
- 题目地址: [https://leetcode-cn.com/problems/delete-operation-for-two-strings](https://leetcode-cn.com/problems/delete-operation-for-two-strings)
- 执行时间: 12 ms 
- 内存消耗: 3.7 MB
- 通过日期: 2019-03-20 15:45

### 题目内容
---
<p>给定两个单词 <em>word1 </em>和 <em>word2</em>，找到使得 <em>word1 </em>和 <em>word2 </em>相同所需的最小步数，每步可以删除任意一个字符串中的一个字符。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> "sea", "eat"
<strong>输出:</strong> 2
<strong>解释:</strong> 第一步将"sea"变为"ea"，第二步将"eat"变为"ea"
</pre>

<p><strong>说明:</strong></p>

<ol>
	<li>给定单词的长度不超过500。</li>
	<li>给定单词中的字符只含有小写字母。</li>
</ol>


### 解法
---
```rust
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


```