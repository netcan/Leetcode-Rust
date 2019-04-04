## Complement of Base 10 Integer :star:
- 题目地址: [https://leetcode-cn.com/problems/complement-of-base-10-integer](https://leetcode-cn.com/problems/complement-of-base-10-integer)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-17 16:04

### 题目内容
---
<p>每个非负整数 <code>N</code> 都有其二进制表示。例如， <code>5</code> 可以被表示为二进制 <code>"101"</code>，<code>11</code> 可以用二进制 <code>"1011"</code> 表示，依此类推。注意，除 <code>N = 0</code> 外，任何二进制表示中都不含前导零。</p>

<p>二进制的反码表示是将每个 <code>1</code> 改为 <code>0</code> 且每个 <code>0</code> 变为 <code>1</code>。例如，二进制数 <code>"101"</code> 的二进制反码为 <code>"010"</code>。</p>

<p>给定十进制数 <code>N</code>，返回其二进制表示的反码所对应的十进制整数。</p>



<ol>
</ol>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>5
<strong>输出：</strong>2
<strong>解释：</strong>5 的二进制表示为 "101"，其二进制反码为 "010"，也就是十进制中的 2 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>7
<strong>输出：</strong>0
<strong>解释：</strong>7 的二进制表示为 "111"，其二进制反码为 "000"，也就是十进制中的 0 。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>10
<strong>输出：</strong>5
<strong>解释：</strong>10 的二进制表示为 "1010"，其二进制反码为 "0101"，也就是十进制中的 5 。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>0 <= N < 10^9</code></li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 { return 1; }
        let mut n_ = 1;
        while n_ < n { n_ <<= 1; }
        !n & (n_ - 1)
    }
}

```