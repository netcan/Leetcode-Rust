# Intersection of Two Arrays :star:
- 题目地址: [https://leetcode-cn.com/problems/intersection-of-two-arrays](https://leetcode-cn.com/problems/intersection-of-two-arrays)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-08 11:43

## 题目内容
---
<p>给定两个数组，编写一个函数来计算它们的交集。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入: </strong>nums1 = [1,2,2,1], nums2 = [2,2]
<strong>输出: </strong>[2]
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong>nums1 = [4,9,5], nums2 = [9,4,9,8,4]
<strong>输出: </strong>[9,4]</pre>

<p><strong>说明:</strong></p>

<ul>
	<li>输出结果中的每个元素一定是唯一的。</li>
	<li>我们可以不考虑输出结果的顺序。</li>
</ul>


## 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a: HashSet<i32> = nums1.iter().cloned().collect();
        let b: HashSet<i32> = nums2.iter().cloned().collect();
        
        a.intersection(&b).cloned().collect()
    }
}

```