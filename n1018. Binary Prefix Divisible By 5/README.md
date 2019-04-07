# Binary Prefix Divisible By 5 :star:
- 题目地址: [https://leetcode-cn.com/problems/binary-prefix-divisible-by-5](https://leetcode-cn.com/problems/binary-prefix-divisible-by-5)
- 执行时间: 8 ms 
- 内存消耗: 3.1 MB
- 通过日期: 2019-03-31 12:44

## 题目内容
<p>给定由若干 <code>0</code> 和 <code>1</code> 组成的数组 <code>A</code>。我们定义 <code>N_i</code>：从 <code>A[0]</code> 到 <code>A[i]</code> 的第 <code>i</code> 个子数组被解释为一个二进制数（从最高有效位到最低有效位）。</p>

<p>返回布尔值列表 <code>answer</code>，只有当 <code>N_i</code> 可以被 <code>5</code> 整除时，答案 <code>answer[i]</code> 为 <code>true</code>，否则为 <code>false</code>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[0,1,1]
<strong>输出：</strong>[true,false,false]
<strong>解释：</strong>
输入数字为 0, 01, 011；也就是十进制中的 0, 1, 3 。只有第一个数可以被 5 整除，因此 answer[0] 为真。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[1,1,1]
<strong>输出：</strong>[false,false,false]
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>[0,1,1,1,1,1]
<strong>输出：</strong>[true,false,false,false,true,false]
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>[1,1,1,0,1]
<strong>输出：</strong>[false,false,false,false,false]
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 30000</code></li>
	<li><code>A[i]</code> 为 <code>0</code> 或 <code>1</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![];
        let mut sum = 0;

        for n in a {
            sum = (sum * 2 + n) % 5;
            ans.push(if sum == 0 { true }
                     else { false });
        }

        ans
    }
}

```