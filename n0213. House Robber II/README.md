### House Robber II :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/house-robber-ii](https://leetcode-cn.com/problems/house-robber-ii)
- 执行时间/Runtime: 24 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 通过日期/Accept Datetime: 2019-03-23 16:22

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 { return *nums.iter().max().unwrap_or(&0); }

        (0..nums.len()).map(|i| {
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
