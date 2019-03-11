
### Middle of the Linked List :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/middle-of-the-linked-list](https://leetcode-cn.com/problems/middle-of-the-linked-list)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 提交日期/Datetime: 2019-03-11 18:34

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pos: i32 = 0;
        let mut p = &mut head;
        while p.is_some() {
            pos += 1;
            p = &mut p.as_mut().unwrap().next;
        }
        pos /= 2;

        p = &mut head;
        while p.is_some() {
            if pos <= 0 { break; }
            pos -= 1;
            p = &mut p.as_mut().unwrap().next;
        }
        p.take()
    }
}


```
