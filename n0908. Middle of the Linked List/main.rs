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

