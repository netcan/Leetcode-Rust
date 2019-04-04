## Longest Univalue Path :star:
- 题目地址: [https://leetcode-cn.com/problems/longest-univalue-path](https://leetcode-cn.com/problems/longest-univalue-path)
- 执行时间: 48 ms 
- 内存消耗: 4.3 MB
- 通过日期: 2019-03-09 23:13

### 题目内容
---
<p>给定一个二叉树，找到最长的路径，这个路径中的每个节点具有相同值。 这条路径可以经过也可以不经过根节点。</p>

<p><strong>注意</strong>：两个节点之间的路径长度由它们之间的边数表示。</p>

<p><strong>示例 1:</strong></p>

<p>输入:</p>

<pre>
              5
             / \
            4   5
           / \   \
          1   1   5
</pre>

<p>输出:</p>

<pre>
2
</pre>

<p><strong>示例 2:</strong></p>

<p>输入:</p>

<pre>
              1
             / \
            4   5
           / \   \
          4   4   5
</pre>

<p>输出:</p>

<pre>
2
</pre>

<p><strong>注意:</strong> 给定的二叉树不超过10000个结点。 树的高度不超过1000。</p>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_length = 0;
        Solution::longest_univalue_path_(&root, &mut max_length);
        max_length
    }

    fn longest_univalue_path_(root: &Option<Rc<RefCell<TreeNode>>>, max_length: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let l = Solution::longest_univalue_path_(&node.left, max_length);
                let r = Solution::longest_univalue_path_(&node.right, max_length);
                let (mut l_len, mut r_len) = (0, 0);
                if let Some(ref left) = node.left {
                    if left.borrow().val == node.val {
                        l_len = l + 1;
                    }
                }
                if let Some(ref right) = node.right {
                    if right.borrow().val == node.val {
                        r_len = r + 1;
                    }
                }
                *max_length = max(*max_length, l_len + r_len);
                max(l_len, r_len)
            },
            None => {
                0
            }
        }
    }
}

```