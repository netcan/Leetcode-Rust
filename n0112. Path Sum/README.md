# Path Sum :star:
- 题目地址: [https://leetcode-cn.com/problems/path-sum](https://leetcode-cn.com/problems/path-sum)
- 执行时间: 4 ms 
- 内存消耗: 3.1 MB
- 通过日期: 2019-03-10 00:06

## 题目内容
<p>给定一个二叉树和一个目标和，判断该树中是否存在根节点到叶子节点的路径，这条路径上所有节点值相加等于目标和。</p>

<p><strong>说明:</strong> 叶子节点是指没有子节点的节点。</p>

<p><strong>示例:</strong> <br>
给定如下二叉树，以及目标和 <code>sum = 22</code>，</p>

<pre>              <strong>5</strong>
             / \
            <strong>4 </strong>  8
           /   / \
          <strong>11 </strong> 13  4
         /  \      \
        7    <strong>2</strong>      1
</pre>

<p>返回 <code>true</code>, 因为存在目标和为 22 的根节点到叶子节点的路径 <code>5->4->11->2</code>。</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Solution::has_path_sum_(&root, sum)
    }

    fn has_path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                if node.left.is_none() &&
                    node.right.is_none() &&
                        diff == 0 { return true; }
                Solution::has_path_sum_(&node.left, diff) ||
                Solution::has_path_sum_(&node.right, diff)
            },
            None => false
        }
    }

}

```