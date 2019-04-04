// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::min;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // dp[day]: 第day天所花费的最小票价
        // dp[day] = min(dp[day-1]+costs[0], dp[day-7]+costs[1], dp[day-30]+costs[2])
        let mut travel = [false; 366];
        let mut dp = [0; 366];
        let (first, last) = (*days.first().unwrap(), *days.last().unwrap());
        for day in days { travel[day as usize] = true; }

        for day in first..=last {
            dp[day as usize] = if travel[day as usize] {
                min(dp[(day - 1) as usize] + costs[0], min(
                        dp[(day - 7).max(0) as usize] + costs[1],
                        dp[(day - 30).max(0) as usize] + costs[2],
                        ))
            } else {
                dp[(day - 1) as usize]
            }
        }

        dp[last as usize]
    }
}
