### Next Greater Node In Linked List :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/next-greater-node-in-linked-list](https://leetcode-cn.com/problems/next-greater-node-in-linked-list)
- 执行时间/Runtime: 424 ms 
- 内存消耗/Mem Usage: 3 MB
- 通过日期/Accept Datetime: 2019-03-31 11:15

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut num = vec![];
        while let Some(node) = head {
            num.push(node.val);
            head = node.next;
        }
        let mut ans = vec![0; num.len()];

        for i in 0..num.len() {
            let mut pos: i32 = -1;
            for j in i + 1..num.len() {
                if num[j] > num[i] {
                    pos = j as i32;
                    break;
                }
            }
            if pos == -1 {
                ans[i] = 0;
            } else {
                ans[i] = num[pos as usize];
            }
        }

        ans
    }
}

```
