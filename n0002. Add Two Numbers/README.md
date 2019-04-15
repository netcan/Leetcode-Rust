# Add Two Numbers :star::star:
- 题目地址: [https://leetcode-cn.com/problems/add-two-numbers](https://leetcode-cn.com/problems/add-two-numbers)
- 执行时间: 8 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-04-13 13:40

## 题目内容
<p>给出两个 <strong>非空</strong> 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 <strong>逆序</strong> 的方式存储的，并且它们的每个节点只能存储 <strong>一位</strong> 数字。</p>

<p>如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。</p>

<p>您可以假设除了数字 0 之外，这两个数都不会以 0 开头。</p>

<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>(2 -> 4 -> 3) + (5 -> 6 -> 4)
<strong>输出：</strong>7 -> 0 -> 8
<strong>原因：</strong>342 + 465 = 807
</pre>


## 解法
```rust
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

```