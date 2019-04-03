## Is Subsequence :star::star:
- 题目地址: [https://leetcode-cn.com/problems/is-subsequence](https://leetcode-cn.com/problems/is-subsequence)
- 执行时间: 4 ms 
- 内存消耗: 4.6 MB
- 通过日期: 2019-03-15 10:02

### 题目内容
<p>给定字符串 <strong>s</strong> 和 <strong>t</strong> ，判断 <strong>s</strong> 是否为 <strong>t</strong> 的子序列。</p>

<p>你可以认为 <strong>s</strong> 和 <strong>t</strong> 中仅包含英文小写字母。字符串 <strong>t</strong> 可能会很长（长度 ~= 500,000），而 <strong>s</strong> 是个短字符串（长度 <=100）。</p>

<p>字符串的一个子序列是原始字符串删除一些（也可以不删除）字符而不改变剩余字符相对位置形成的新字符串。（例如，<code>"ace"</code>是<code>"abcde"</code>的一个子序列，而<code>"aec"</code>不是）。</p>

<p><strong>示例 1:</strong><br />
<strong>s</strong> = <code>"abc"</code>, <strong>t</strong> = <code>"ahbgdc"</code></p>

<p>返回 <code>true</code>.</p>

<p><strong>示例 2:</strong><br />
<strong>s</strong> = <code>"axc"</code>, <strong>t</strong> = <code>"ahbgdc"</code></p>

<p>返回 <code>false</code>.</p>

<p><strong>后续挑战</strong> <strong>:</strong></p>

<p>如果有大量输入的 S，称作S1, S2, ... , Sk 其中 k >= 10亿，你需要依次检查它们是否为 T 的子序列。在这种情况下，你会怎样改变代码？</p>

<p><strong>致谢:</strong></p>

<p>特别感谢<strong> </strong><a href="https://leetcode.com/pbrother/">@pbrother </a>添加此问题并且创建所有测试用例。</p>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 { return true; }
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let (mut i, mut j) = (0, 0);
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
                if i == s.len() { return true; }
            }
            j += 1;
        }
        false
    }
}


```
