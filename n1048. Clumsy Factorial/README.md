
### Clumsy Factorial :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/clumsy-factorial](https://leetcode-cn.com/problems/clumsy-factorial)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.6 MB
- 通过日期/Accept Datetime: 2019-03-13 22:55

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut op_idx = 0;
        let mut seq: Vec<i32> = vec![n];

        // 先算乘除
        for i in (1..=n-1).rev() {
            if op_idx == 0 {
                let j = seq.pop().unwrap();
                seq.push(j * i);
            } else if op_idx == 1 {
                let j = seq.pop().unwrap();
                seq.push(j / i);
            } else {
                seq.push(i);
            }
            op_idx = (op_idx + 1) % 4;
        }

        // 后算加减
        seq.iter().skip(1).enumerate().fold(seq[0], |acc, (idx, x)| {
            if idx % 2 == 0 { acc + x } // +
            else { acc - x } // -
        })
    }
}

```
