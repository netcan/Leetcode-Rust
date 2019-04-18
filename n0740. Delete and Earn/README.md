# Delete and Earn :star::star:
- 题目地址: [https://leetcode-cn.com/problems/delete-and-earn](https://leetcode-cn.com/problems/delete-and-earn)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-17 20:03

## 题目内容
<p>给定一个整数数组 <code>nums</code> ，你可以对它进行一些操作。</p>

<p>每次操作中，选择任意一个 <code>nums[i]</code> ，删除它并获得 <code>nums[i]</code> 的点数。之后，你必须删除<strong>每个</strong>等于 <code>nums[i] - 1</code> 或 <code>nums[i] + 1</code> 的元素。</p>

<p>开始你拥有 0 个点数。返回你能通过这些操作获得的最大点数。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> nums = [3, 4, 2]
<strong>输出:</strong> 6
<strong>解释:</strong> 
删除 4 来获得 4 个点数，因此 3 也被删除。
之后，删除 2 来获得 2 个点数。总共获得 6 个点数。
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> nums = [2, 2, 3, 3, 3, 4]
<strong>输出:</strong> 9
<strong>解释:</strong> 
删除 3 来获得 3 个点数，接着要删除两个 2 和 4 。
之后，再次删除 3 获得 3 个点数，再次删除 3 获得 3 个点数。
总共获得 9 个点数。
</pre>

<p><strong>注意:</strong></p>

<ul>
	<li><code>nums</code>的长度最大为<code>20000</code>。</li>
	<li>每个整数<code>nums[i]</code>的大小都在<code>[1, 10000]</code>范围内。</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // 打家劫舍问题
        let mut gold = [0; 10001];
        for &n in &nums {
            gold[n as usize] += n;
        }
        let mut dp = [0; 10001];
        let (mut take, mut max_earn) = (0, 0);
        for i in 1..=10000usize {
            dp[i] = if i > 1 {
                take = take.max(dp[i - 2]); // dp[0..=i-2].max()
                take + gold[i]
            } else {
                gold[i]
            };
            max_earn = max_earn.max(dp[i]);
        }

        max_earn
    }
}


```