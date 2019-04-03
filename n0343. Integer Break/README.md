## Integer Break :star::star:
- 题目地址: [https://leetcode-cn.com/problems/integer-break](https://leetcode-cn.com/problems/integer-break)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-07 15:06

### 题目内容
<p>给定一个正整数 <em>n</em>，将其拆分为<strong>至少</strong>两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入: </strong>2
<strong>输出: </strong>1
<strong>解释: </strong>2 = 1 + 1, 1 × 1 = 1。</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong>10
<strong>输出: </strong>36
<strong>解释: </strong>10 = 3 + 3 + 4, 3 × 3 × 4 = 36。</pre>

<p><strong>说明: </strong>你可以假设 <em>n </em>不小于 2 且不大于 58。</p>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut dp = [0; 60];
        for x in (2..=n) {
            for i in (1..=x/2) {
                let j = x - i;
                dp[x] = max(dp[x], max(max(max(i * j, dp[i] * dp[j]), dp[i] * j), i * dp[j]));
            }
        }
        dp[n] as i32
    }
}


```
