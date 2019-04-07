# Minimum Cost For Tickets :star::star:
- 题目地址: [https://leetcode-cn.com/problems/minimum-cost-for-tickets](https://leetcode-cn.com/problems/minimum-cost-for-tickets)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-04 20:26

## 题目内容
<p>在一个火车旅行很受欢迎的国度，你提前一年计划了一些火车旅行。在接下来的一年里，你要旅行的日子将以一个名为 <code>days</code> 的数组给出。每一项是一个从 <code>1</code> 到 <code>365</code> 的整数。</p>

<p>火车票有三种不同的销售方式：</p>

<ul>
	<li>一张为期一天的通行证售价为 <code>costs[0]</code> 美元；</li>
	<li>一张为期七天的通行证售价为 <code>costs[1]</code> 美元；</li>
	<li>一张为期三十天的通行证售价为 <code>costs[2]</code> 美元。</li>
</ul>

<p>通行证允许数天无限制的旅行。 例如，如果我们在第 2 天获得一张为期 7 天的通行证，那么我们可以连着旅行 7 天：第 2 天、第 3 天、第 4 天、第 5 天、第 6 天、第 7 天和第 8 天。</p>

<p>返回你想要完成在给定的列表 <code>days</code> 中列出的每一天的旅行所需要的最低消费。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>days = [1,4,6,7,8,20], costs = [2,7,15]
<strong>输出：</strong>11
<strong>解释： </strong>
例如，这里有一种购买通行证的方法，可以让你完成你的旅行计划：
在第 1 天，你花了 costs[0] = $2 买了一张为期 1 天的通行证，它将在第 1 天生效。
在第 3 天，你花了 costs[1] = $7 买了一张为期 7 天的通行证，它将在第 3, 4, ..., 9 天生效。
在第 20 天，你花了 costs[0] = $2 买了一张为期 1 天的通行证，它将在第 20 天生效。
你总共花了 $11，并完成了你计划的每一天旅行。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>days = [1,2,3,4,5,6,7,8,9,10,30,31], costs = [2,7,15]
<strong>输出：</strong>17
<strong>解释：
</strong>例如，这里有一种购买通行证的方法，可以让你完成你的旅行计划： 
在第 1 天，你花了 costs[2] = $15 买了一张为期 30 天的通行证，它将在第 1, 2, ..., 30 天生效。
在第 31 天，你花了 costs[0] = $2 买了一张为期 1 天的通行证，它将在第 31 天生效。 
你总共花了 $17，并完成了你计划的每一天旅行。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= days.length <= 365</code></li>
	<li><code>1 <= days[i] <= 365</code></li>
	<li><code>days</code> 按顺序严格递增</li>
	<li><code>costs.length == 3</code></li>
	<li><code>1 <= costs[i] <= 1000</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::min;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // dp[day]: 第day天所花费的最小票价
        // dp[day] = min(dp[day-1]+costs[0], dp[day-7]+costs[1], dp[day-30]+costs[2])
        let mut travel = [false; 366];
        let mut dp = [0; 366];
        let (first, last) = (*days.first().unwrap(), *days.last().unwrap());
        for day in days { travel[day as usize] = true; }

        for day in first..=last {
            dp[day as usize] = if travel[day as usize] {
                min(dp[(day - 1) as usize] + costs[0], min(
                        dp[(day - 7).max(0) as usize] + costs[1],
                        dp[(day - 30).max(0) as usize] + costs[2],
                        ))
            } else {
                dp[(day - 1) as usize]
            }
        }

        dp[last as usize]
    }
}

```