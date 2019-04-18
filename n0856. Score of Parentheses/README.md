# Score of Parentheses :star::star:
- 题目地址: [https://leetcode-cn.com/problems/score-of-parentheses](https://leetcode-cn.com/problems/score-of-parentheses)
- 执行时间: 0 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-04-18 19:55

## 题目内容
<p>给定一个平衡括号字符串 <code>S</code>，按下述规则计算该字符串的分数：</p>

<ul>
	<li><code>()</code> 得 1 分。</li>
	<li><code>AB</code> 得 <code>A + B</code> 分，其中 A 和 B 是平衡括号字符串。</li>
	<li><code>(A)</code> 得 <code>2 * A</code> 分，其中 A 是平衡括号字符串。</li>
</ul>



<p><strong>示例 1：</strong></p>

<pre><strong>输入： </strong>"()"
<strong>输出： </strong>1
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入： </strong>"(())"
<strong>输出： </strong>2
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入： </strong>"()()"
<strong>输出： </strong>2
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入： </strong>"(()(()))"
<strong>输出： </strong>6
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>S</code> 是平衡括号字符串，且只含有 <code>(</code> 和 <code>)</code> 。</li>
	<li><code>2 <= S.length <= 50</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        Solution::score_of_parentheses_(&s)
    }

    fn score_of_parentheses_(s: &str) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut level = 0;
        let mut score = 0;
        while right < s.len() {
            if s.as_bytes()[right] == '(' as u8 {
                level += 1;
                if level == 1 { left = right; }
            } else {
                level -= 1;
                if level == 0 { // 找到一对括号
                    if left + 1 < right { // 嵌套 * 2
                        score += Solution::score_of_parentheses_(&s[left + 1 .. right]) * 2;
                    } else {
                        score += 1;
                    }
                }
            }
            right += 1;
        }
        score
    }
}


```