# Longest Arithmetic Sequence :star::star:
- 题目地址: [https://leetcode-cn.com/problems/longest-arithmetic-sequence](https://leetcode-cn.com/problems/longest-arithmetic-sequence)
- 执行时间: 268 ms 
- 内存消耗: 171.4 MB
- 通过日期: 2019-04-14 12:26

## 题目内容
<p>给定一个整数数组 <code>A</code>，返回 <code>A</code> 中最长等差子序列的<strong>长度</strong>。</p>

<p>回想一下，<code>A</code> 的子序列是列表 <code>A[i_1], A[i_2], ..., A[i_k]</code> 其中 <code>0 <= i_1 < i_2 < ... < i_k <= A.length - 1</code>。并且如果 <code>B[i+1] - B[i]</code>( <code>0 <= i < B.length - 1</code>) 的值都相同，那么序列 <code>B</code> 是等差的。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[3,6,9,12]
<strong>输出：</strong>4
<strong>解释： </strong>
整个数组是公差为 3 的等差数列。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[9,4,7,2,10]
<strong>输出：</strong>3
<strong>解释：</strong>
最长的等差子序列是 [4,7,10]。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>[20,1,15,3,10,5,8]
<strong>输出：</strong>4
<strong>解释：</strong>
最长的等差子序列是 [20,15,10,5]。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>2 <= A.length <= 2000</code></li>
	<li><code>0 <= A[i] <= 10000</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        let max_diff = *a.iter().max().unwrap() - *a.iter().min().unwrap();
        if max_diff == 0 { return a.len() as i32; }
        // dp[i][diff]: 以第i个数为结尾，[0..i]这个子序列以diff为等差的最大长度
        // dp[i][diff] = max(dp[i][diff], dp[j][diff] + 1) for j in 0..i
        // 考虑到diff可能为负数，所以加个偏移，使其下标diff_idx >= 0
        let mut dp = vec![vec![1; (max_diff * 2) as usize + 1]; a.len()];
        let mut ans = 1;
        for i in 1..a.len() {
            for j in 0..i {
                let diff = a[i] - a[j] + max_diff; // 令diff >= 0
                dp[i][diff as usize] = dp[i][diff as usize].max(dp[j][diff as usize] + 1);
                ans=ans.max(dp[i][diff as usize]);
            }
        }

        ans
    }
}

```