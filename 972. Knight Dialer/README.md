
### Knight Dialer :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/knight-dialer](https://leetcode-cn.com/problems/knight-dialer)
- 执行时间/Runtime: 16 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 通过日期/Accept Datetime: 2019-03-06 18:27

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let n: usize = n as usize;
        let NEXT_STEP = vec![
            vec![4, 6],    // 0
            vec![6, 8],    // 1
            vec![7, 9],    // 2
            vec![4, 8],    // 3
            vec![0, 3, 9], // 4
            vec![],        // 5
            vec![0, 1, 7], // 6
            vec![2, 6],    // 7
            vec![1, 3],    // 8
            vec![4, 2],    // 9
        ];
        let mut dp = [[0; 10], [1; 10]];
        for s in 2..=n {
            for d in 0..=9 {
                dp[s & 1][d] = 0;
                for ns in &NEXT_STEP[d] {
                    dp[s & 1][d] += dp[(s+1) & 1][*ns];
                    dp[s & 1][d] %= 1000000007;
                }
            }
        }
        // println!("{:#?}", dp);
        
        dp[n & 1].iter().fold(0, |a, &b| { (a+b) % 1000000007 })
    }
}


```
