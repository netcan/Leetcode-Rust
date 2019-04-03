## Second Minimum Node In a Binary Tree :star:
- 题目地址: [https://leetcode-cn.com/problems/second-minimum-node-in-a-binary-tree](https://leetcode-cn.com/problems/second-minimum-node-in-a-binary-tree)
- 执行时间: 0 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-06 23:57

### 题目内容
<p>给定一个非空特殊的二叉树，每个节点都是正数，并且每个节点的子节点数量只能为 <code>2</code> 或 <code>0</code>。如果一个节点有两个子节点的话，那么这个节点的值不大于它的子节点的值。 </p>

<p>给出这样的一个二叉树，你需要输出所有节点中的<strong>第二小的值。</strong>如果第二小的值不存在的话，输出 -1 <strong>。</strong></p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> 
    2
   / \
  2   5
     / \
    5   7

<strong>输出:</strong> 5
<strong>说明:</strong> 最小的值是 2 ，第二小的值是 5 。
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> 
    2
   / \
  2   2

<strong>输出:</strong> -1
<strong>说明:</strong> 最小的值是 2, 但是不存在第二小的值。
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match &root {
            Some(node) => Solution::find_second_minimum_value_(&root, node.borrow().val),
            None => -1
        }
    }

    fn find_second_minimum_value_(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                if node.val > val {
                    return node.val;
                }
                let l = Solution::find_second_minimum_value_(&node.left, val);
                let r = Solution::find_second_minimum_value_(&node.right, val);
                if l < 0 {
                    return r;
                }
                if r < 0 {
                    return l;
                }
                cmp::min(l, r)
            },
            None => {
                -1
            }
        }
    }
}

```
