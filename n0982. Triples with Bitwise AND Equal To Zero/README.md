## Triples with Bitwise AND Equal To Zero :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/triples-with-bitwise-and-equal-to-zero](https://leetcode-cn.com/problems/triples-with-bitwise-and-equal-to-zero)
- 执行时间: 144 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-03-15 17:52

### 题目内容
---
<p>给定一个整数数组 <code>A</code>，找出索引为 (i, j, k) 的三元组，使得：</p>

<ul>
	<li><code>0 <= i < A.length</code></li>
	<li><code>0 <= j < A.length</code></li>
	<li><code>0 <= k < A.length</code></li>
	<li><code>A[i] & A[j] & A[k] == 0</code>，其中 <code>&</code> 表示按位与（AND）操作符。</li>
</ul>



<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>[2,1,3]
<strong>输出：</strong>12
<strong>解释：</strong>我们可以选出如下 i, j, k 三元组：
(i=0, j=0, k=1) : 2 & 2 & 1
(i=0, j=1, k=0) : 2 & 1 & 2
(i=0, j=1, k=1) : 2 & 1 & 1
(i=0, j=1, k=2) : 2 & 1 & 3
(i=0, j=2, k=1) : 2 & 3 & 1
(i=1, j=0, k=0) : 1 & 2 & 2
(i=1, j=0, k=1) : 1 & 2 & 1
(i=1, j=0, k=2) : 1 & 2 & 3
(i=1, j=1, k=0) : 1 & 1 & 2
(i=1, j=2, k=0) : 1 & 3 & 2
(i=2, j=0, k=1) : 3 & 2 & 1
(i=2, j=1, k=0) : 3 & 1 & 2
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 1000</code></li>
	<li><code>0 <= A[i] < 2^16</code></li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let max_num = nums.iter().skip(1).fold(nums[0], |a, b| { a.max(*b) } ) as usize;
        // dp[n] 为A[i] & A[j] = n的个数
        let mut dp = vec![0; max_num + 1];
        for ai in &nums {
            for aj in &nums { dp[(ai & aj) as usize] += 1; }
        }

        let mut ans = 0;
        for a in &nums {
            for b in (0..=max_num as i32) {
                if a & b == 0 {
                    ans += dp[b as usize];
                }
            }
        }

        ans
    }
}


```