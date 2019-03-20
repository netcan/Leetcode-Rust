### Longest Increasing Subsequence :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/longest-increasing-subsequence](https://leetcode-cn.com/problems/longest-increasing-subsequence)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-20 19:15

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut longest = Vec::with_capacity(nums.len());

        for num in nums {
            match longest.binary_search(&num) {
                Err(pos) => {
                    if pos < longest.len() {
                        longest[pos] = num;
                    }  else  {
                        longest.push(num);
                    }
                },
                _ => {}
            }
        }

        longest.len() as i32
    }
}


```
