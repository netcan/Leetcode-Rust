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

