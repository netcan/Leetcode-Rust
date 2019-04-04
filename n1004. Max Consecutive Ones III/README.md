## Max Consecutive Ones III :star::star:
- 题目地址: [https://leetcode-cn.com/problems/max-consecutive-ones-iii](https://leetcode-cn.com/problems/max-consecutive-ones-iii)
- 执行时间: 12 ms 
- 内存消耗: 3.3 MB
- 通过日期: 2019-03-18 11:35

### 题目内容
---
<p>给定一个由若干 <code>0</code> 和 <code>1</code> 组成的数组 <code>A</code>，我们最多可以将 <code>K</code> 个值从 0 变成 1 。</p>

<p>返回仅包含 1 的最长（连续）子数组的长度。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>A = [1,1,1,0,0,0,1,1,1,1,0], K = 2
<strong>输出：</strong>6
<strong>解释： </strong>
[1,1,1,0,0,<strong>1</strong>,1,1,1,1,<strong>1</strong>]
粗体数字从 0 翻转到 1，最长的子数组长度为 6。</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>A = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], K = 3
<strong>输出：</strong>10
<strong>解释：</strong>
[0,0,1,1,<strong>1</strong>,<strong>1</strong>,1,1,1,<strong>1</strong>,1,1,0,0,0,1,1,1,1]
粗体数字从 0 翻转到 1，最长的子数组长度为 10。</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 20000</code></li>
	<li><code>0 <= K <= A.length</code></li>
	<li><code>A[i]</code> 为 <code>0</code> 或 <code>1</code> </li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let mut zeroes = VecDeque::new();
        let mut ans = 0;
        let mut i = 0;
        for j in 0..a.len() {
            if a[j] == 0 {
                zeroes.push_back(j);
                if zeroes.len() > k as usize {
                    i = zeroes.pop_front().unwrap() + 1;
                }
            }

            ans = ans.max(j - i + 1);
        }

        ans as i32
    }
}


```