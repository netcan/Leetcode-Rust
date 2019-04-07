# Squares of a Sorted Array :star:
- 题目地址: [https://leetcode-cn.com/problems/squares-of-a-sorted-array](https://leetcode-cn.com/problems/squares-of-a-sorted-array)
- 执行时间: 16 ms 
- 内存消耗: 3.1 MB
- 通过日期: 2019-03-11 22:29

## 题目内容
<p>给定一个按非递减顺序排序的整数数组 <code>A</code>，返回每个数字的平方组成的新数组，要求也按非递减顺序排序。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[-4,-1,0,3,10]
<strong>输出：</strong>[0,1,9,16,100]
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[-7,-3,2,3,11]
<strong>输出：</strong>[4,9,9,49,121]
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 10000</code></li>
	<li><code>-10000 <= A[i] <= 10000</code></li>
	<li><code>A</code> 已按非递减顺序排序。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut a = a.iter().map(|&x| x * x).collect::<Vec<i32>>();
        a.sort();
        a
    }
}

```