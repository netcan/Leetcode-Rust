## Sort Array By Parity II :star:
- 题目地址: [https://leetcode-cn.com/problems/sort-array-by-parity-ii](https://leetcode-cn.com/problems/sort-array-by-parity-ii)
- 执行时间: 20 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-03-06 11:17

### 题目内容
---
<p>给定一个非负整数数组 <code>A</code>， A 中一半整数是奇数，一半整数是偶数。</p>

<p>对数组进行排序，以便当 <code>A[i]</code> 为奇数时，<code>i</code> 也是奇数；当 <code>A[i]</code> 为偶数时， <code>i</code> 也是偶数。</p>

<p>你可以返回任何满足上述条件的数组作为答案。</p>



<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>[4,2,5,7]
<strong>输出：</strong>[4,5,2,7]
<strong>解释：</strong>[4,7,2,5]，[2,5,4,7]，[2,7,4,5] 也会被接受。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>2 <= A.length <= 20000</code></li>
	<li><code>A.length % 2 == 0</code></li>
	<li><code>0 <= A[i] <= 1000</code></li>
</ol>




### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(a.len());
        unsafe { ret.set_len(a.len()); }
        let (mut i, mut j) = (0, 1);
        for n in &a {
            if n & 1 == 1 {
                ret[j] = *n;
                j += 2;
            } else {
                ret[i] = *n;
                i += 2;
            }
        }
        ret
    }
}


```