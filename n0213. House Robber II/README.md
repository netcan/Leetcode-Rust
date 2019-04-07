# House Robber II :star::star:
- 题目地址: [https://leetcode-cn.com/problems/house-robber-ii](https://leetcode-cn.com/problems/house-robber-ii)
- 执行时间: 4 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-23 16:32

## 题目内容
<p>你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都<strong>围成一圈，</strong>这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，<strong>如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警</strong>。</p>

<p>给定一个代表每个房屋存放金额的非负整数数组，计算你<strong>在不触动警报装置的情况下，</strong>能够偷窃到的最高金额。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> [2,3,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> [1,2,3,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
     偷窃到的最高金额 = 1 + 3 = 4 。</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 { return *nums.iter().max().unwrap_or(&0); }

        (0..=2).map(|i| {
            let mut vis = vec![-1; nums.len()]; // 记录偷过的最大价值？
            Solution::dfs(&nums, &mut vis, i, i)
        }).max().unwrap_or(0)
    }
    fn dfs(nums: &Vec<i32>, vis: &mut Vec<i32>, starti: usize, i: usize) -> i32 { // 从第start_i家开始偷，目前偷到第i家
        let n = nums.len();
        if (i + 1) % n == starti { return 0; } // 偷到头了，回溯
        if vis[i] != -1 { return vis[i]; }

        let mut value = nums[i];
        for step in 2..n {
            if (step + i) % n <= i { break; }
            value = value.max(nums[i] + Solution::dfs(nums, vis, starti, step + i));
        }

        vis[i] = value;
        value
    }
}


```