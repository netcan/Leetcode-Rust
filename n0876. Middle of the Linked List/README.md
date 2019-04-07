# Middle of the Linked List :star:
- 题目地址: [https://leetcode-cn.com/problems/middle-of-the-linked-list](https://leetcode-cn.com/problems/middle-of-the-linked-list)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-11 18:34

## 题目内容
<p>给定一个带有头结点 <code>head</code> 的非空单链表，返回链表的中间结点。</p>

<p>如果有两个中间结点，则返回第二个中间结点。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[1,2,3,4,5]
<strong>输出：</strong>此列表中的结点 3 (序列化形式：[3,4,5])
返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
注意，我们返回了一个 ListNode 类型的对象 ans，这样：
ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next = NULL.
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[1,2,3,4,5,6]
<strong>输出：</strong>此列表中的结点 4 (序列化形式：[4,5,6])
由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。
</pre>



<p><strong>提示：</strong></p>

<ul>
	<li>给定链表的结点数介于 <code>1</code> 和 <code>100</code> 之间。</li>
</ul>


## 解法
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