## Sum Root to Leaf Numbers :star::star:
- 题目地址: [https://leetcode-cn.com/problems/sum-root-to-leaf-numbers](https://leetcode-cn.com/problems/sum-root-to-leaf-numbers)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-10 00:17

### 题目内容
---
<p>给定一个二叉树，它的每个结点都存放一个 <code>0-9</code> 的数字，每条从根到叶子节点的路径都代表一个数字。</p>

<p>例如，从根到叶子节点路径 <code>1->2->3</code> 代表数字 <code>123</code>。</p>

<p>计算从根到叶子节点生成的所有数字之和。</p>

<p><strong>说明:</strong> 叶子节点是指没有子节点的节点。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> [1,2,3]
    1
   / \
  2   3
<strong>输出:</strong> 25
<strong>解释:</strong>
从根到叶子节点路径 <code>1->2</code> 代表数字 <code>12</code>.
从根到叶子节点路径 <code>1->3</code> 代表数字 <code>13</code>.
因此，数字总和 = 12 + 13 = <code>25</code>.</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> [4,9,0,5,1]
    4
   / \
  9   0
 / \
5   1
<strong>输出:</strong> 1026
<strong>解释:</strong>
从根到叶子节点路径 <code>4->9->5</code> 代表数字 495.
从根到叶子节点路径 <code>4->9->1</code> 代表数字 491.
从根到叶子节点路径 <code>4->0</code> 代表数字 40.
因此，数字总和 = 495 + 491 + 40 = <code>1026</code>.</pre>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;
        Solution::sum_numbers_(&root, 0, &mut ret);
        ret
    }
    fn sum_numbers_(root: &Option<Rc<RefCell<TreeNode>>>, num: i32, sum: &mut i32) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let num = num * 10 + node.val;
                if node.left.is_none() && node.right.is_none() {
                    *sum += num;
                }
                Solution::sum_numbers_(&node.left, num, sum);
                Solution::sum_numbers_(&node.right, num, sum);
            },
            None => {}
        }

    }
}


```