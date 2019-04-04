## Coin Change :star::star:
- 题目地址: [https://leetcode-cn.com/problems/coin-change](https://leetcode-cn.com/problems/coin-change)
- 执行时间: 12 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-04-04 11:37

### 题目内容
---
<p>给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回 <code>-1</code>。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入: </strong>coins = <code>[1, 2, 5]</code>, amount = <code>11</code>
<strong>输出: </strong><code>3</code> 
<strong>解释:</strong> 11 = 5 + 5 + 1</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong>coins = <code>[2]</code>, amount = <code>3</code>
<strong>输出: </strong>-1</pre>

<p><strong>说明</strong>:<br>
你可以认为每种硬币的数量是无限的。</p>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // dp[m]: 凑到金额m所需要的最少金币数
        // dp[m] = min(dp[m - coins[i]] + 1, dp[m]), dp[coins[i]] = 1
        if amount == 0 { return 0; }
        let mut dp = vec![amount + 1; amount as usize + 1];
        for c in &coins {
            if *c <= amount { dp[*c as usize] = 1; }
        }

        for i in 0..coins.len() {
            for m in coins[i]..=amount {
                dp[m as usize] = dp[m as usize].min(dp[(m - coins[i]) as usize] + 1);
            }
        }

        if dp[amount as usize] > amount { -1 }
        else { dp[amount as usize] }
    }
}


```