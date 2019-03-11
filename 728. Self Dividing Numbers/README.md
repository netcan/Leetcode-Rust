
### Self Dividing Numbers :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/self-dividing-numbers](https://leetcode-cn.com/problems/self-dividing-numbers)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 提交日期/Datetime: 2019-03-06 09:28

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let ret = (left..=right).filter(|&x| {
            let mut _x = x;
            let mut dividing = true;
            while(_x > 0) {
                if(_x % 10 == 0 || (x % (_x % 10)) != 0) {
                    dividing = false;
                    break;
                }
                _x /= 10;
            }
            dividing
        }).collect();
        ret
    }
}


```
