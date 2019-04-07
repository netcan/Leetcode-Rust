# Smallest Integer Divisible by K :star::star:
- 题目地址: [https://leetcode-cn.com/problems/smallest-integer-divisible-by-k](https://leetcode-cn.com/problems/smallest-integer-divisible-by-k)
- 执行时间: 32 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-02 18:37

## 题目内容
<p>给定正整数 <code>K</code>，你需要找出可以被 K 整除的、仅包含数字 <strong>1</strong> 的最小正整数 N。</p>

<p>返回 <code>N</code> 的长度。如果不存在这样的 <code>N</code>，就返回 <code>-1</code>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>1
<strong>输出：</strong>1
<strong>解释：</strong>最小的答案是 N = 1，其长度为 1。</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>2
<strong>输出：</strong>-1
<strong>解释：</strong>不存在可被 2 整除的正整数 N 。</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>3
<strong>输出：</strong>3
<strong>解释：</strong>最小的答案是 N = 111，其长度为 3。</pre>



<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= K <= 10^5</code></li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let (mut num, mut len) = (1, 1); 
        while num % k != 0 && len < 100000 {
            num = (num * 10 + 1) % k;
            len += 1;
        }

        if num % k == 0 { len } 
        else { -1 }
    }
}


```