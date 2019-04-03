## Partition Array Into Three Parts With Equal Sum :star:
- 题目地址: [https://leetcode-cn.com/problems/partition-array-into-three-parts-with-equal-sum](https://leetcode-cn.com/problems/partition-array-into-three-parts-with-equal-sum)
- 执行时间: 12 ms 
- 内存消耗: 3.6 MB
- 通过日期: 2019-04-02 16:10

### 题目内容
<p>给定一个整数数组 <code>A</code>，只有我们可以将其划分为三个和相等的非空部分时才返回 <code>true</code>，否则返回 <code>false</code>。</p>

<p>形式上，如果我们可以找出索引 <code>i+1 < j</code> 且满足 <code>(A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... + A[j-1] == A[j] + A[j-1] + ... + A[A.length - 1])</code> 就可以将数组三等分。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输出：</strong>[0,2,1,-6,6,-7,9,1,2,0,1]
<strong>输出：</strong>true
<strong>解释：</strong>0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[0,2,1,-6,6,7,9,-1,2,0,1]
<strong>输出：</strong>false
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>[3,3,6,5,-2,2,5,1,-9,4]
<strong>输出：</strong>true
<strong>解释：</strong>3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>3 <= A.length <= 50000</code></li>
	<li><code>-10000 <= A[i] <= 10000</code></li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        if a.len() < 3 { return false; }
        let sum = a.iter().sum::<i32>() / 3;

        let mut p = [0; 3]; // equal part
        let mut i = 0;
        for k in 0..3 {
            while i < a.len() {
                p[k] += a[i];
                if p[k] == sum { i += 1; break; }
                i += 1;
            }
        }
        return p[0] == p[1] && p[1] == p[2] && p[2] == sum;
    }
}

```
