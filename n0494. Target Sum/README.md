# Target Sum :star::star:
- 题目地址: [https://leetcode-cn.com/problems/target-sum](https://leetcode-cn.com/problems/target-sum)
- 执行时间: 104 ms 
- 内存消耗: 2.8 MB
- 通过日期: 2019-04-11 19:26

## 题目内容
<p>给定一个非负整数数组，a1, a2, ..., an, 和一个目标数，S。现在你有两个符号 <code>+</code> 和 <code>-</code>。对于数组中的任意一个整数，你都可以从 <code>+</code> 或 <code>-</code>中选择一个符号添加在前面。</p>

<p>返回可以使最终数组和为目标数 S 的所有添加符号的方法数。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> nums: [1, 1, 1, 1, 1], S: 3
<strong>输出:</strong> 5
<strong>解释:</strong> 

-1+1+1+1+1 = 3
+1-1+1+1+1 = 3
+1+1-1+1+1 = 3
+1+1+1-1+1 = 3
+1+1+1+1-1 = 3

一共有5种方法让最终目标和为3。
</pre>

<p><strong>注意:</strong></p>

<ol>
	<li>数组的长度不会超过20，并且数组中的值全为正数。</li>
	<li>初始的数组的和不会超过1000。</li>
	<li>保证返回的最终结果为32位整数。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashMap;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new(); // 记忆化
        Solution::target_sum_ways(&nums, s, -1, &mut memo)
    }

    fn target_sum_ways(nums: &Vec<i32>, s: i32, i: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if i >= nums.len() as i32 - 1 {
            if s == 0 { return 1; }
            else { return 0 };
        }
        if memo.contains_key(&(i, s)) { return memo[&(i, s)]; }

        let ways = Solution::target_sum_ways(nums, s - nums[(i + 1) as usize], i + 1, memo) + // +
            Solution::target_sum_ways(nums, s + nums[(i + 1) as usize], i + 1, memo); // -

        memo.insert((i, s), ways);
        ways
    }

}


```