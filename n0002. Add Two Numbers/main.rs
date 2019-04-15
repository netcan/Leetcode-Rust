// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut p, mut q) = (&l1, &l2);
        let mut head: Option<Box<ListNode>> = None;
        let mut cur = &mut head;
        
        let mut shift = 0; // 进位
        while (p.is_some() || q.is_some()) {
            let d = if p.is_some() { p.as_ref().unwrap().val }
                    else { 0 } + 
                    if q.is_some() { q.as_ref().unwrap().val }
                    else { 0 } +
                    shift;
            
            *cur = Some(Box::new(ListNode::new(d % 10)));
            cur = &mut cur.as_mut().unwrap().next;
            shift = d / 10;
            
            if p.is_some() { p = &p.as_ref().unwrap().next; }
            if q.is_some() { q = &q.as_ref().unwrap().next; }
        }
        
        if shift > 0 {
            *cur = Some(Box::new(ListNode::new(shift)));
        }
        
        head
    }
}
