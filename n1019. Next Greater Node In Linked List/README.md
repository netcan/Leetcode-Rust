## Next Greater Node In Linked List :star::star:
- 题目地址: [https://leetcode-cn.com/problems/next-greater-node-in-linked-list](https://leetcode-cn.com/problems/next-greater-node-in-linked-list)
- 执行时间: 72 ms 
- 内存消耗: 2.9 MB
- 通过日期: 2019-03-31 12:57

### 题目内容
---
<p>给出一个以头节点 <code>head</code> 作为第一个节点的链表。链表中的节点分别编号为：<code>node_1, node_2, node_3, ...</code> 。</p>

<p>每个节点都可能有下一个更大值（<em>next larger</em> <strong>value</strong>）：对于 <code>node_i</code>，如果其 <code>next_larger(node_i)</code> 是 <code>node_j.val</code>，那么就有 <code>j > i</code> 且  <code>node_j.val > node_i.val</code>，而 <code>j</code> 是可能的选项中最小的那个。如果不存在这样的 <code>j</code>，那么下一个更大值为 <code>0</code> 。</p>

<p>返回整数答案数组 <code>answer</code>，其中 <code>answer[i] = next_larger(node_{i+1})</code> 。</p>

<p><strong><em>注意：</em></strong>在下面的示例中，诸如 <code>[2,1,5]</code> 这样的<strong>输入</strong>（不是输出）是链表的序列化表示，其头节点的值为 2，第二个节点值为 1，第三个节点值为 5 。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[2,1,5]
<strong>输出：</strong>[5,5,0]
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[2,7,4,3,5]
<strong>输出：</strong>[7,0,5,5,0]
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>[1,7,5,1,9,2,5,1]
<strong>输出：</strong>[7,9,9,9,0,5,0,0]
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li>对于链表中的每个节点，<code>1 <= node.val <= 10^9</code></li>
	<li>给定列表的长度在 <code>[0, 10000]</code> 范围内</li>
</ol>


### 解法
---
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
        let mut stack = vec![];

        for i in 0..num.len() {
            while stack.len() > 0 && num[*stack.last().unwrap()] < num[i] {
                ans[stack.pop().unwrap()] = num[i];
            }
            stack.push(i);
        }

        ans
    }
}

```