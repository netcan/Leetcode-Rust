# Coin Change 2 :star::star:
- 题目地址: [https://leetcode-cn.com/problems/coin-change-2](https://leetcode-cn.com/problems/coin-change-2)
- 执行时间: 12 ms 
- 内存消耗: 8.4 MB
- 通过日期: 2019-04-04 14:41

## 题目内容
<p>给定不同面额的硬币和一个总金额。写出函数来计算可以凑成总金额的硬币组合数。假设每一种面额的硬币有无限个。 </p>



<ul>
</ul>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> amount = 5, coins = [1, 2, 5]
<strong>输出:</strong> 4
<strong>解释:</strong> 有四种方式可以凑成总金额:
5=5
5=2+2+1
5=2+1+1+1
5=1+1+1+1+1
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> amount = 3, coins = [2]
<strong>输出:</strong> 0
<strong>解释:</strong> 只用面额2的硬币不能凑成总金额3。
</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入:</strong> amount = 10, coins = [10] 
<strong>输出:</strong> 1
</pre>



<p><strong>注意</strong><strong>:</strong></p>

<p>你可以假设：</p>

<ul>
	<li>0 <= amount (总金额) <= 5000</li>
	<li>1 <= coin (硬币面额) <= 5000</li>
	<li>硬币种类不超过 500 种</li>
	<li>结果符合 32 位符号整数</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter::repeat;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // dp[i][m]: 前i种硬币凑成m的个数
        // dp[i][m] = dp[i][m-coins[i]] + dp[i-1][m]
        // dp[i][0] = 1, dp[0][m] = 0

        let mut dp: Vec<Vec<i32>> =
            repeat(vec![0; amount as usize + 1]).take(coins.len() + 1).collect();
        for i in 0..=coins.len() { dp[i][0] = 1; }

        for i in 0..coins.len() {
            for m in 1..=amount {
                dp[i + 1][m as usize] = if m >= coins[i] {
                    dp[i + 1][(m - coins[i]) as usize]
                } else {
                    0
                } + dp[i][m as usize];
            }
        }
        dp[coins.len()][amount as usize]
    }
}


```