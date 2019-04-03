## Edit Distance :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/edit-distance](https://leetcode-cn.com/problems/edit-distance)
- 执行时间: 8 ms 
- 内存消耗: 3.4 MB
- 通过日期: 2019-03-20 18:55

### 题目内容
<p>给定两个单词 <em>word1</em> 和 <em>word2</em>，计算出将 <em>word1</em> 转换成 <em>word2 </em>所使用的最少操作数 。</p>

<p>你可以对一个单词进行如下三种操作：</p>

<ol>
	<li>插入一个字符</li>
	<li>删除一个字符</li>
	<li>替换一个字符</li>
</ol>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> word1 = "horse", word2 = "ros"
<strong>输出:</strong> 3
<strong>解释:</strong> 
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> word1 = "intention", word2 = "execution"
<strong>输出:</strong> 5
<strong>解释:</strong> 
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    // dp[i][j]: word[0..i) -> word[0..j) step
    // dp[i][j] = dp[i-1][j-1] if word1[i-1] == word2[j-1]
    // else = min(
    //  dp[i-1][j-1] + 1, change word1[i-1] to word2[j-1]
    //  dp[i-1][j] + 1, remove word1[i-1]
    //  dp[i][j-1] + 1, add word2[j-1]
    // )
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
        let mut dp:Vec<Vec<i32>> = iter::repeat(vec![0; word2.len() + 1]).take(word1.len() + 1).collect();
        for i in 0..=m { dp[i][0] = i as i32; }
        for j in 0..=n { dp[0][j] = j as i32; }

        for (i, &c1) in word1.iter().enumerate() {
            for (j, &c2) in word2.iter().enumerate() {
                dp[i + 1][j + 1] = if c1 == c2 {
                    dp[i][j]
                } else {
                    dp[i][j].min(dp[i + 1][j]).min(dp[i][j+1]) + 1
                }
            }
        }

        dp[m][n]
    }
}


```
