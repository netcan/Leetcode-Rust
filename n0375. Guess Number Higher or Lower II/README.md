# Guess Number Higher or Lower II :star::star:
- 题目地址: [https://leetcode-cn.com/problems/guess-number-higher-or-lower-ii](https://leetcode-cn.com/problems/guess-number-higher-or-lower-ii)
- 执行时间: 4 ms 
- 内存消耗: 2.7 MB
- 通过日期: 2019-03-17 15:54

## 题目内容
<p>我们正在玩一个猜数游戏，游戏规则如下：</p>

<p>我从 <strong>1 </strong>到 <strong>n</strong> 之间选择一个数字，你来猜我选了哪个数字。</p>

<p>每次你猜错了，我都会告诉你，我选的数字比你的大了或者小了。</p>

<p>然而，当你猜了数字 x 并且猜错了的时候，你需要支付金额为 x 的现金。直到你猜到我选的数字，你才算赢得了这个游戏。</p>

<p><strong>示例:</strong></p>

<pre>n = 10, 我选择了8.

第一轮: 你猜我选择的数字是5，我会告诉你，我的数字更大一些，然后你需要支付5块。
第二轮: 你猜是7，我告诉你，我的数字更大一些，你支付7块。
第三轮: 你猜是9，我告诉你，我的数字更小一些，你支付9块。

游戏结束。8 就是我选的数字。

你最终要支付 5 + 7 + 9 = 21 块钱。
</pre>

<p>给定 <strong>n ≥ 1，</strong>计算你至少需要拥有多少现金才能确保你能赢得这个游戏。</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        // dp[i][j]: i到j的最小代价
        // dp[i][j] = min(k + max(dp[i][k-1], dp[k+1][j])) for k in i..j
        // dp[i][i+1] = i

        let n: usize = n as usize;
        let mut dp: Vec<Vec<i32>> =
            iter::repeat(vec![0; n as usize + 1])
            .take(n as usize + 1).collect();

        for i in 1..n { dp[i][i+1] = i as i32; }

        for step in 1..n {
            for i in 1..=n - step {
                let j = i + step;
                let mut min_money = -1;
                for k in i+1..j {
                    let money = k as i32 + dp[i][k-1].max(dp[k+1][j]);
                    if min_money == -1 || min_money > money { min_money = money; }
                }
                if min_money != -1 { dp[i][j] = min_money; }
            }
        }

        dp[1][n]
    }
}

```