### Convert to Base -2 :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/convert-to-base-2](https://leetcode-cn.com/problems/convert-to-base-2)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-03-31 12:48

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 { return 0.to_string(); }
        let mut ans = String::new();
        while n != 0 {
            let mut reminder = n % -2;
            n /= -2;
            if reminder < 0 {
                reminder += 2;
                n += 1;
            }
            ans = reminder.to_string() + &ans;
        }

        ans
    }
}

```
