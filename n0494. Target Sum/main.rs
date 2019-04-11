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

