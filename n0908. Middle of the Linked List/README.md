### Middle of the Linked List :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/middle-of-the-linked-list](https://leetcode-cn.com/problems/middle-of-the-linked-list)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-11 17:13

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pos: i32 = 0;
        let mut p = &mut head;
        while let Some(ref mut node) = *p {
            pos += 1;
            p = &mut node.next;
        }
        pos /= 2;

        while let Some(ref mut node) = head {
            if pos <= 0 { break; }
            head = node.next.take();
            pos -= 1;
        }
        head
    }
}


```
