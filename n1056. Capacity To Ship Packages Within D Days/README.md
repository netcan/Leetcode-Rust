### Capacity To Ship Packages Within D Days :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days](https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days)
- 执行时间/Runtime: 16 ms 
- 内存消耗/Mem Usage: 3.3 MB
- 通过日期/Accept Datetime: 2019-03-17 16:25

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let max_weight:i32 = *weights.iter().max().unwrap();
        let sum_weight:i32 = weights.iter().sum();

        let (mut lb, mut ub) = (max_weight, sum_weight + 1);
        while ub - lb > 1 {
            let mid = (lb + ub) / 2;
            if Solution::check_weight(&weights, mid, d) { ub = mid; }
            else { lb = mid; }
        }

        if Solution::check_weight(&weights, lb, d) { lb }
        else { ub }
    }

    fn check_weight(weights: &Vec<i32>, min_weight: i32, d: i32) -> bool {
        let mut curd = 0;
        let mut weight = 0;
        for w in weights {
            if weight + w > min_weight {
                weight = *w;
                curd += 1;
            } else {
                weight += w;
            }
            if curd >= d { return false; };
        }

        if curd <= d { return true; }
        else { return false; }
    }
}

```
