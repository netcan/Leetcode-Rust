
### Minimum Cost to Merge Stones :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/minimum-cost-to-merge-stones](https://leetcode-cn.com/problems/minimum-cost-to-merge-stones)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 2.8 MB
- 通过日期/Accept Datetime: 2019-03-15 23:34

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, K: i32) -> i32 {
        // 检查是否有解
        let n = stones.len();
        if (2 - K - n as i32) % (1 - K) != 0 { return -1; }
        else if n == 1 { return 0; }

        let mut sumCoin = vec![0; n + 1];
        for (i, stone) in stones.iter().enumerate() {
            sumCoin[i + 1] = sumCoin[i] + stone;
        }

        // dp[i][j][k]: 将i到j区间的石头合并成k块需要的最小代价
        // dp[i][j][k] = min(dp[i][j][k], dp[i][t][k-1] + dp[t+1][j][1]) k > 2
        // dp[i][j][1] = dp[i][j][K] + sumCoin[i][j]
        let mut dp: Vec<Vec<Vec<i32>>> = iter::repeat(
            iter::repeat(vec![100000; K as usize + 1]).take(n + 1).collect()
        ).take(n + 1).collect();

        for i in 1..=n { dp[i][i][1] = 0; }

        for step in 1..n as usize {
            for i in 1..=(n - step) as usize {
                let j = i + step;
                for k in 2..=K as usize {
                    for t in i..j {
                        dp[i][j][k] = dp[i][j][k].min(dp[i][t][k-1] + dp[t+1][j][1]);
                    }
                }
                // k块合成1块等于k块的代价 + 最后所有的和
                dp[i][j][1] = dp[i][j][K as usize] + sumCoin[j] - sumCoin[i - 1];
            }
        }

        dp[1][n][1]
    }
}

```
