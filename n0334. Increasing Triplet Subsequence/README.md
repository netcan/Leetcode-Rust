### Increasing Triplet Subsequence :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/increasing-triplet-subsequence](https://leetcode-cn.com/problems/increasing-triplet-subsequence)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-03-20 20:03

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 { return false; }
        let mut increasing = [i32::max_value(); 3]; // O(1) space

        for num in nums {
            match increasing.binary_search(&num) {
                Err(pos) => {
                    if pos >= 2 { return true; }
                    increasing[pos] = num;
                },
                _ => {}
            }
        }
        false
    }
}


```
