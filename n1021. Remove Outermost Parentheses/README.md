# Remove Outermost Parentheses :star:
- 题目地址: [https://leetcode-cn.com/problems/remove-outermost-parentheses](https://leetcode-cn.com/problems/remove-outermost-parentheses)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-07 12:20

## 题目内容
<p>有效括号字符串为空 <code>("")</code>、<code>"(" + A + ")"</code> 或 <code>A + B</code>，其中 <code>A</code> 和 <code>B</code> 都是有效的括号字符串，<code>+</code> 代表字符串的连接。例如，<code>""</code>，<code>"()"</code>，<code>"(())()"</code> 和 <code>"(()(()))"</code> 都是有效的括号字符串。</p>

<p>如果有效字符串 <code>S</code> 非空，且不存在将其拆分为 <code>S = A+B</code> 的方法，我们称其为<strong>原语（primitive）</strong>，其中 <code>A</code> 和 <code>B</code> 都是非空有效括号字符串。</p>

<p>给出一个非空有效字符串 <code>S</code>，考虑将其进行原语化分解，使得：<code>S = P_1 + P_2 + ... + P_k</code>，其中 <code>P_i</code> 是有效括号字符串原语。</p>

<p>对 <code>S</code> 进行原语化分解，删除分解中每个原语字符串的最外层括号，返回 <code>S</code> 。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>"(()())(())"
<strong>输出：</strong>"()()()"
<strong>解释：
</strong>输入字符串为 "(()())(())"，原语化分解得到 "(()())" + "(())"，
删除每个部分中的最外层括号后得到 "()()" + "()" = "()()()"。</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>"(()())(())(()(()))"
<strong>输出：</strong>"()()()()(())"
<strong>解释：</strong>
输入字符串为 "(()())(())(()(()))"，原语化分解得到 "(()())" + "(())" + "(()(()))"，
删除每隔部分中的最外层括号后得到 "()()" + "()" + "()(())" = "()()()()(())"。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>"()()"
<strong>输出：</strong>""
<strong>解释：</strong>
输入字符串为 "()()"，原语化分解得到 "()" + "()"，
删除每个部分中的最外层括号后得到 "" + "" = ""。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>S.length <= 10000</code></li>
	<li><code>S[i]</code> 为 <code>"("</code> 或 <code>")"</code></li>
	<li><code>S</code> 是一个有效括号字符串</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let s = s.into_bytes();
        let mut lev = 0;
        let mut primitive = String::new();
        for &p in &s {
            if p == '(' as u8 {
                if lev > 0 { primitive += "("; }
                lev += 1;
            } else { // ')'
                lev -= 1;
                if lev > 0 { primitive += ")"; }
            }
        }

        primitive
    }
}

```