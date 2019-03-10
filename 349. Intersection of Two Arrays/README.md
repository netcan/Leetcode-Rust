
### Intersection of Two Arrays :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/intersection-of-two-arrays](https://leetcode-cn.com/problems/intersection-of-two-arrays)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 提交日期/Datetime: 2019-03-08 11:43

```rust
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a: HashSet<i32> = nums1.iter().cloned().collect();
        let b: HashSet<i32> = nums2.iter().cloned().collect();
        
        a.intersection(&b).cloned().collect()
    }
}
```
