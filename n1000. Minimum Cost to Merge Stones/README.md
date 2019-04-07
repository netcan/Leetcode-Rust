# Minimum Cost to Merge Stones :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/minimum-cost-to-merge-stones](https://leetcode-cn.com/problems/minimum-cost-to-merge-stones)
- 执行时间: 4 ms 
- 内存消耗: 2.8 MB
- 通过日期: 2019-03-15 23:34

## 题目内容
<p>有 <code>N</code> 堆石头排成一排，第 <code>i</code> 堆中有 <code>stones[i]</code> 块石头。</p>

<p>每次<em>移动（move）</em>需要将<strong>连续的</strong> <code>K</code> 堆石头合并为一堆，而这个移动的成本为这 <code>K</code> 堆石头的总数。</p>

<p>找出把所有石头合并成一堆的最低成本。如果不可能，返回 <code>-1</code> 。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>stones = [3,2,4,1], K = 2
<strong>输出：</strong>20
<strong>解释：</strong>
从 [3, 2, 4, 1] 开始。
合并 [3, 2]，成本为 5，剩下 [5, 4, 1]。
合并 [4, 1]，成本为 5，剩下 [5, 5]。
合并 [5, 5]，成本为 10，剩下 [10]。
总成本 20，这是可能的最小值。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>stones = [3,2,4,1], K = 3
<strong>输出：</strong>-1
<strong>解释：</strong>任何合并操作后，都会剩下 2 堆，我们无法再进行合并。所以这项任务是不可能完成的。.
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>stones = [3,5,1,2,6], K = 3
<strong>输出：</strong>25
<strong>解释：</strong>
从 [3, 5, 1, 2, 6] 开始。
合并 [5, 1, 2]，成本为 8，剩下 [3, 8, 6]。
合并 [3, 8, 6]，成本为 17，剩下 [17]。
总成本 25，这是可能的最小值。
</pre>



<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= stones.length <= 30</code></li>
	<li><code>2 <= K <= 30</code></li>
	<li><code>1 <= stones[i] <= 100</code></li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, K: i32) -> i32 {
        // 检查是否有解
        let n = stones.len();
        if (2 - K - n as i32) % (1 - K) != 0 { return -1; }
        else if n == 1 { return 0; }

        let mut sumCoin = vec![0; n + 1];
        for (i, stone) in stones.iter().enumerate() {
            sumCoin[i + 1] = sumCoin[i] + stone;
        }

        // dp[i][j][k]: 将i到j区间的石头合并成k块需要的最小代价
        // dp[i][j][k] = min(dp[i][j][k], dp[i][t][k-1] + dp[t+1][j][1]) k > 2
        // dp[i][j][1] = dp[i][j][K] + sumCoin[i][j]
        let mut dp: Vec<Vec<Vec<i32>>> = iter::repeat(
            iter::repeat(vec![100000; K as usize + 1]).take(n + 1).collect()
        ).take(n + 1).collect();

        for i in 1..=n { dp[i][i][1] = 0; }

        for step in 1..n as usize {
            for i in 1..=(n - step) as usize {
                let j = i + step;
                for k in 2..=K as usize {
                    for t in i..j {
                        dp[i][j][k] = dp[i][j][k].min(dp[i][t][k-1] + dp[t+1][j][1]);
                    }
                }
                // k块合成1块等于k块的代价 + 最后所有的和
                dp[i][j][1] = dp[i][j][K as usize] + sumCoin[j] - sumCoin[i - 1];
            }
        }

        dp[1][n][1]
    }
}

```