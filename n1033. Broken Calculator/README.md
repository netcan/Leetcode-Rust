
### Broken Calculator :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/broken-calculator](https://leetcode-cn.com/problems/broken-calculator)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-15 09:31

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn broken_calc(x: i32, mut y: i32) -> i32 {
        if (x >= y) { return x - y; }
        let mut step = 0;
        while y >= 0 {
            if y % 2 == 0 {
                y /= 2;
                step += 1;
            } else {
                y = y / 2 + 1;
                step += 2;
            }
            if x >= y { return step + x - y; }
        }
        step
    }
}


```
