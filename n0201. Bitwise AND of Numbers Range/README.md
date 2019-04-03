## Bitwise AND of Numbers Range :star::star:
- 题目地址: [https://leetcode-cn.com/problems/bitwise-and-of-numbers-range](https://leetcode-cn.com/problems/bitwise-and-of-numbers-range)
- 执行时间: 24 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-07 12:05

### 题目内容
<p>给定范围 [m, n]，其中 0 <= m <= n <= 2147483647，返回此范围内所有数字的按位与（包含 m, n 两端点）。</p>

<p><strong>示例 1: </strong></p>

<pre><strong>输入:</strong> [5,7]
<strong>输出:</strong> 4</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> [0,1]
<strong>输出:</strong> 0</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let m: u32 = m as u32;
        let n: u32 = n as u32;
        let diff = n - m + 1;
        let mut bit_and = 1;
        while bit_and < diff {
            bit_and <<= 1;
        }
        (m & !(bit_and - 1) & n) as i32
    }
}


```
