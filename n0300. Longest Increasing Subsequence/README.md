## Longest Increasing Subsequence :star::star:
- 题目地址: [https://leetcode-cn.com/problems/longest-increasing-subsequence](https://leetcode-cn.com/problems/longest-increasing-subsequence)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-20 19:15

### 题目内容
<p>给定一个无序的整数数组，找到其中最长上升子序列的长度。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> <code>[10,9,2,5,3,7,101,18]
</code><strong>输出: </strong>4 
<strong>解释: </strong>最长的上升子序列是 <code>[2,3,7,101]，</code>它的长度是 <code>4</code>。</pre>

<p><strong>说明:</strong></p>

<ul>
	<li>可能会有多种最长上升子序列的组合，你只需要输出对应的长度即可。</li>
	<li>你算法的时间复杂度应该为 O(<em>n<sup>2</sup></em>) 。</li>
</ul>

<p><strong>进阶:</strong> 你能将算法的时间复杂度降低到 O(<em>n</em> log <em>n</em>) 吗?</p>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut longest = Vec::with_capacity(nums.len());

        for num in nums {
            match longest.binary_search(&num) {
                Err(pos) => {
                    if pos < longest.len() {
                        longest[pos] = num;
                    }  else  {
                        longest.push(num);
                    }
                },
                _ => {}
            }
        }

        longest.len() as i32
    }
}


```
