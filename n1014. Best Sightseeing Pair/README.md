# Best Sightseeing Pair :star::star:
- 题目地址: [https://leetcode-cn.com/problems/best-sightseeing-pair](https://leetcode-cn.com/problems/best-sightseeing-pair)
- 执行时间: 8 ms 
- 内存消耗: 3.5 MB
- 通过日期: 2019-04-02 17:10

## 题目内容
<p>给定正整数数组 <code>A</code>，<code>A[i]</code> 表示第 <code>i</code> 个观光景点的评分，并且两个景点 <code>i</code> 和 <code>j</code> 之间的距离为 <code>j - i</code>。</p>

<p>一对景点（<code>i < j</code>）组成的观光组合的得分为（<code>A[i] + A[j] + i - j</code>）：景点的评分之和<strong>减去</strong>它们两者之间的距离。</p>

<p>返回一对观光景点能取得的最高分。</p>



<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>[8,1,5,2,6]
<strong>输出：</strong>11
<strong>解释：</strong>i = 0, j = 2, <code>A[i] + A[j] + i - j = 8 + 5 + 0 - 2 = 11</code>
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>2 <= A.length <= 50000</code></li>
	<li><code>1 <= A[i] <= 1000</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let (mut max_score, mut max_pair) = (0, 0);
        for i in 0..a.len() {
            max_score = (a[i] - i as i32 + max_pair).max(max_score);
            max_pair = max_pair.max(a[i] + i as i32);
        }

        max_score
    }
}


```