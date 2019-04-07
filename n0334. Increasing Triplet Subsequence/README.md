# Increasing Triplet Subsequence :star::star:
- 题目地址: [https://leetcode-cn.com/problems/increasing-triplet-subsequence](https://leetcode-cn.com/problems/increasing-triplet-subsequence)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-20 20:03

## 题目内容
---
<p>给定一个未排序的数组，判断这个数组中是否存在长度为 3 的递增子序列。</p>

<p>数学表达式如下:</p>

<blockquote>如果存在这样的 <em>i, j, k, </em> 且满足 0 ≤ <em>i</em> < <em>j</em> < <em>k</em> ≤ <em>n</em>-1，<br>
使得 <em>arr[i]</em> < <em>arr[j]</em> < <em>arr[k] </em>，返回 true ; 否则返回 false 。</blockquote>

<p><strong>说明:</strong> 要求算法的时间复杂度为 O(<em>n</em>)，空间复杂度为 O(<em>1</em>) 。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入: </strong>[1,2,3,4,5]
<strong>输出: </strong>true
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong>[5,4,3,2,1]
<strong>输出: </strong>false</pre>


## 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 { return false; }
        let mut increasing = [i32::max_value(); 3]; // O(1) space

        for num in nums {
            match increasing.binary_search(&num) {
                Err(pos) => {
                    if pos >= 2 { return true; }
                    increasing[pos] = num;
                },
                _ => {}
            }
        }
        false
    }
}


```