### Binary Prefix Divisible By 5 :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-prefix-divisible-by-5](https://leetcode-cn.com/problems/binary-prefix-divisible-by-5)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 3.1 MB
- 通过日期/Accept Datetime: 2019-03-31 12:44

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![];
        let mut sum = 0;

        for n in a {
            sum = (sum * 2 + n) % 5;
            ans.push(if sum == 0 { true }
                     else { false });
        }

        ans
    }
}

```
