# Minimum Depth of Binary Tree :star:
- 题目地址: [https://leetcode-cn.com/problems/minimum-depth-of-binary-tree](https://leetcode-cn.com/problems/minimum-depth-of-binary-tree)
- 执行时间: 4 ms 
- 内存消耗: 3.2 MB
- 通过日期: 2019-03-08 17:55

## 题目内容
<p>给定一个二叉树，找出其最小深度。</p>

<p>最小深度是从根节点到最近叶子节点的最短路径上的节点数量。</p>

<p><strong>说明:</strong> 叶子节点是指没有子节点的节点。</p>

<p><strong>示例:</strong></p>

<p>给定二叉树 <code>[3,9,20,null,null,15,7]</code>,</p>

<pre>    3
   / \
  9  20
    /  \
   15   7</pre>

<p>返回它的最小深度  2.</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let None = root {
            return 0;
        }
        let mut que: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
        que.push_back((root, 1));
        while !que.is_empty() {
            let (node, depth) = que.pop_front().unwrap();
            match node {
                Some(node) => {
                    let node = node.borrow();
                    if let None = node.left {
                        if let None = node.right {
                            return depth;
                        }
                    }

                    if let Some(_) = node.left {
                        que.push_back((node.left.clone(), depth + 1));
                    }
                    
                    if let Some(_) = node.right {
                        que.push_back((node.right.clone(), depth + 1));
                    }

                },
                None => {
                    return depth;
                }
            }

        }
        0

    }
}


```