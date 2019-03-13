
### Maximize Sum Of Array After K Negations :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/maximize-sum-of-array-after-k-negations](https://leetcode-cn.com/problems/maximize-sum-of-array-after-k-negations)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-03-13 22:41

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn largest_sum_after_k_negations(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort();
        let b = a.clone();

        let mut j = 0;
        for _ in 0..k {
            a[j] = -a[j];
            if b[j] < 0 && j + 1 < a.len() {
                if b[j + 1] <= 0 || -b[j] > b[j + 1] {
                    j += 1;
                }
            }
        }
        a.iter().sum()
    }
}

```
