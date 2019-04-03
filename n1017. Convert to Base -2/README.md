## Convert to Base -2 :star::star:
- 题目地址: [https://leetcode-cn.com/problems/convert-to-base-2](https://leetcode-cn.com/problems/convert-to-base-2)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-31 12:48

### 题目内容
<p>给出数字 <code>N</code>，返回由若干 <code>"0"</code> 和 <code>"1"</code>组成的字符串，该字符串为 <code>N</code> 的<strong>负二进制（<code>base -2</code>）</strong>表示。</p>

<p>除非字符串就是 <code>"0"</code>，否则返回的字符串中不能含有前导零。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>2
<strong>输出：</strong>"110"
<strong>解释：</strong>(-2) ^ 2 + (-2) ^ 1 = 2
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>3
<strong>输出：</strong>"111"
<strong>解释：</strong>(-2) ^ 2 + (-2) ^ 1 + (-2) ^ 0 = 3
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>4
<strong>输出：</strong>"100"
<strong>解释：</strong>(-2) ^ 2 = 4
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>0 <= N <= 10^9</code></li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 { return 0.to_string(); }
        let mut ans = String::new();
        while n != 0 {
            let mut reminder = n % -2;
            n /= -2;
            if reminder < 0 {
                reminder += 2;
                n += 1;
            }
            ans = reminder.to_string() + &ans;
        }

        ans
    }
}

```
