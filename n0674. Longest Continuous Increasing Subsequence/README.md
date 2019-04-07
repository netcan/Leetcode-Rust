# Longest Continuous Increasing Subsequence :star:
- 题目地址: [https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence](https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence)
- 执行时间: 0 ms 
- 内存消耗: 2.7 MB
- 通过日期: 2019-03-15 12:09

## 题目内容
<p>给定一个未经排序的整数数组，找到最长且<strong>连续</strong>的的递增序列。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> [1,3,5,4,7]
<strong>输出:</strong> 3
<strong>解释:</strong> 最长连续递增序列是 [1,3,5], 长度为3。
尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为5和7在原数组里被4隔开。 
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> [2,2,2,2,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 最长连续递增序列是 [2], 长度为1。
</pre>

<p><strong>注意：</strong>数组长度不会超过10000。</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let (mut length, mut max_length) = (1, 1);
        nums.iter().skip(1).fold(nums[0], |a, b| {
            if a < *b { length += 1; max_length = max(max_length, length) }
            else { length = 1; }
            *b
        });
        max_length
    }
}


```