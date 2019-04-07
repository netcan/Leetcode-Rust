# Burst Balloons :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/burst-balloons](https://leetcode-cn.com/problems/burst-balloons)
- 执行时间: 4 ms 
- 内存消耗: 2.6 MB
- 通过日期: 2019-03-16 00:31

## 题目内容
<p>有 <code>n</code> 个气球，编号为<code>0</code> 到 <code>n-1</code>，每个气球上都标有一个数字，这些数字存在数组 <code>nums</code> 中。</p>

<p>现在要求你戳破所有的气球。每当你戳破一个气球 <code>i</code> 时，你可以获得 <code>nums[left] * nums[i] * nums[right]</code> 个硬币。 这里的 <code>left</code> 和 <code>right</code> 代表和 <code>i</code> 相邻的两个气球的序号。注意当你戳破了气球 <code>i</code> 后，气球 <code>left</code> 和气球 <code>right</code> 就变成了相邻的气球。</p>

<p>求所能获得硬币的最大数量。</p>

<p><strong>说明:</strong></p>

<ul>
	<li>你可以假设 <code>nums[-1] = nums[n] = 1</code>，但注意它们不是真实存在的所以并不能被戳破。</li>
	<li>0 ≤ <code>n</code> ≤ 500, 0 ≤ <code>nums[i]</code> ≤ 100</li>
</ul>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> <code>[3,1,5,8]</code>
<strong>输出:</strong> <code>167 
<strong>解释: </strong></code>nums = [3,1,5,8] --> [3,5,8] -->   [3,8]   -->  [8]  --> []
     coins =  3*1*5      +  3*5*8    +  1*3*8      + 1*8*1   = 167
</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        // dp[i][j]: 从i到j获得的最大硬币数量
        // coin[k]: 戳爆第k个气球得分
        // dp[i][j] = max(dp[i][j], dp[i][k - 1] + coin[i-1]*coin[k]*coin[j+1] + dp[k+1][j])
        let n = nums.len();
        if n == 0 { return 0; }
        else if n == 1 { return nums[0]; }

        let mut coin = vec![1]; coin.append(&mut nums); coin.push(1);

        let mut dp:Vec<Vec<i32>> =
            iter::repeat(vec![0; n + 1]).take(n + 1).collect();
        for i in 1..=n { dp[i][i] = coin[i-1] * coin[i] * coin[i+1]; }

        for step in 1..n {
            for i in 1..=n-step {
                let j = i + step;
                for k in i..=j {
                    let left = if k - 1 < i { 0 } else { dp[i][k - 1] };
                    let right = if k + 1 > j { 0 } else { dp[k + 1][j] };
                    dp[i][j] = dp[i][j].max(left + right + coin[i-1] * coin[k] * coin[j+1]);
                }
            }
        }
        dp[1][n]
    }
}

```