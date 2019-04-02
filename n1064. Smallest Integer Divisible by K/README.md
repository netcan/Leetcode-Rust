### Smallest Integer Divisible by K :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/smallest-integer-divisible-by-k](https://leetcode-cn.com/problems/smallest-integer-divisible-by-k)
- 执行时间/Runtime: 304 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 通过日期/Accept Datetime: 2019-04-02 18:35

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let (mut num, mut len) = (1, 1); 
        while num % k != 0 && len < 1000000 {
            num = (num * 10 + 1) % k;
            len += 1;
        }

        if num % k == 0 { len } 
        else { -1 }
    }
}


```
