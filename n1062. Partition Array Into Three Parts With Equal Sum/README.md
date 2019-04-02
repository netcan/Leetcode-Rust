### Partition Array Into Three Parts With Equal Sum :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/partition-array-into-three-parts-with-equal-sum](https://leetcode-cn.com/problems/partition-array-into-three-parts-with-equal-sum)
- 执行时间/Runtime: 12 ms 
- 内存消耗/Mem Usage: 3.6 MB
- 通过日期/Accept Datetime: 2019-04-02 16:10

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        if a.len() < 3 { return false; }
        let sum = a.iter().sum::<i32>() / 3;

        let mut p = [0; 3]; // equal part
        let mut i = 0;
        for k in 0..3 {
            while i < a.len() {
                p[k] += a[i];
                if p[k] == sum { i += 1; break; }
                i += 1;
            }
        }
        return p[0] == p[1] && p[1] == p[2] && p[2] == sum;
    }
}

```
