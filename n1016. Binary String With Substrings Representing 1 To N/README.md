## Binary String With Substrings Representing 1 To N :star::star:
- 题目地址: [https://leetcode-cn.com/problems/binary-string-with-substrings-representing-1-to-n](https://leetcode-cn.com/problems/binary-string-with-substrings-representing-1-to-n)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-02 19:28

### 题目内容
<p>给定一个二进制字符串 <code>S</code>（一个仅由若干 '0' 和 '1' 构成的字符串）和一个正整数 <code>N</code>，如果对于从 <code>1</code> 到 <code>N</code> 的每个整数 <code>X</code>，其二进制表示都是 <code>S</code> 的子串，就返回 <code>true</code>，否则返回 <code>false</code>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>S = "0110", N = 3
<strong>输出：</strong>true
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>S = "0110", N = 4
<strong>输出：</strong>false
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= S.length <= 1000</code></li>
	<li><code>1 <= N <= 10^9</code></li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        // 最多有s_len ^ 2种子串
        if s.len() * s.len() < n as usize { return false; }
        for i in 1..=n {
            if let None = s.find(&format!("{:b}", i)) {
                return false;
            }
        }
        true
    }
}


```
