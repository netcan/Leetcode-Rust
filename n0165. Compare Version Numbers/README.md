### Compare Version Numbers :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/compare-version-numbers](https://leetcode-cn.com/problems/compare-version-numbers)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-21 15:14

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        if version1.len() <= 0 || version2.len() <= 0 { return 0; }

        let version1: Vec<&str> = version1.split('.').collect();
        let version2: Vec<&str> = version2.split('.').collect();
        let mut i = 0;
        while i < version1.len() || i < version2.len() {
            let num1: u32 = if i < version1.len() {
                version1[i].parse().unwrap()
            } else { 0 };

            let num2: u32 = if i < version2.len() {
                version2[i].parse().unwrap()
            } else { 0 };
            if num1 > num2 { return 1; }
            else if num1 < num2 { return -1; }
            i += 1;
        }

        0
    }
}


```
