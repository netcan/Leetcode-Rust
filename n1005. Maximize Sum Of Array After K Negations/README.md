## Maximize Sum Of Array After K Negations :star:
- 题目地址: [https://leetcode-cn.com/problems/maximize-sum-of-array-after-k-negations](https://leetcode-cn.com/problems/maximize-sum-of-array-after-k-negations)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-13 22:41

### 题目内容
---
<p>给定一个整数数组 A，我们<strong>只能</strong>用以下方法修改该数组：我们选择某个个索引 <code>i</code> 并将 <code>A[i]</code> 替换为 <code>-A[i]</code>，然后总共重复这个过程 <code>K</code> 次。（我们可以多次选择同一个索引 <code>i</code>。）</p>

<p>以这种方式修改数组后，返回数组可能的最大和。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>A = [4,2,3], K = 1
<strong>输出：</strong>5
<strong>解释：</strong>选择索引 (1,) ，然后 A 变为 [4,-2,3]。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>A = [3,-1,0,2], K = 3
<strong>输出：</strong>6
<strong>解释：</strong>选择索引 (1, 2, 2) ，然后 A 变为 [3,1,0,2]。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>A = [2,-3,-1,5,-4], K = 2
<strong>输出：</strong>13
<strong>解释：</strong>选择索引 (1, 4) ，然后 A 变为 [2,3,-1,5,4]。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 10000</code></li>
	<li><code>1 <= K <= 10000</code></li>
	<li><code>-100 <= A[i] <= 100</code></li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn largest_sum_after_k_negations(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort();
        let b = a.clone();

        let mut j = 0;
        for _ in 0..k {
            a[j] = -a[j];
            if b[j] < 0 && j + 1 < a.len() {
                if b[j + 1] <= 0 || -b[j] > b[j + 1] {
                    j += 1;
                }
            }
        }
        a.iter().sum()
    }
}

```