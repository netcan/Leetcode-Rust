
### Assign Cookies :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/assign-cookies](https://leetcode-cn.com/problems/assign-cookies)
- 执行时间/Runtime: 12 ms 
- 内存消耗/Mem Usage: 3.1 MB
- 通过日期/Accept Datetime: 2019-03-05 20:22

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut ret = 0;
        let mut j = 0;
        for ss in &s {
            if j >= g.len() {
                break;
            }
            if *ss >= g[j] {
                ret += 1;
                j += 1;
            }
        }

        ret
    }
}



```
