# Number of Longest Increasing Subsequence :star::star:
- 题目地址: [https://leetcode-cn.com/problems/number-of-longest-increasing-subsequence](https://leetcode-cn.com/problems/number-of-longest-increasing-subsequence)
- 执行时间: 24 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-21 12:17

## 题目内容
<p>给定一个未排序的整数数组，找到最长递增子序列的个数。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> [1,3,5,4,7]
<strong>输出:</strong> 2
<strong>解释:</strong> 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> [2,2,2,2,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 最长递增子序列的长度是1，并且存在5个子序列的长度为1，因此输出5。
</pre>

<p><strong>注意:</strong> 给定的数组长度不超过 2000 并且结果一定是32位有符号整数。</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return nums.len() as i32; }
        let mut dp = vec![1; nums.len()]; // dp[i]以nums[i]结尾的LIS
        let mut dp_len = vec![1; nums.len()];
        let (mut max_len, mut cnt) = (1, 0);
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[j] > nums[i] {
                    if dp[i] + 1 > dp[j] {
                        dp[j] = dp[i] + 1;
                        dp_len[j] = dp_len[i];
                    } else if dp[i]+ 1 == dp[j]  {
                        dp_len[j] += dp_len[i];
                    }
                    max_len = max_len.max(dp[j]);
                } 
            }
        }
        let mut cnt = 0;
        for (i, &len) in dp.iter().enumerate() {
            if len == max_len { cnt += dp_len[i]; }
        }
        cnt
    }
}


```