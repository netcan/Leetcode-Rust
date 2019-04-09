# Flatten Binary Tree to Linked List :star::star:
- 题目地址: [https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list](https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-09 18:48

## 题目内容
<p>给定一个二叉树，<a href="https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95/8010757" target="_blank">原地</a>将它展开为链表。</p>

<p>例如，给定二叉树</p>

<pre>    1
   / \
  2   5
 / \   \
3   4   6</pre>

<p>将其展开为：</p>

<pre>1
 \
  2
   \
    3
     \
      4
       \
        5
         \
          6</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // 把左树插到右树上
        match &root {
            Some(node) => {
                let mut node = node.borrow_mut();
                Solution::flatten(&mut node.left);
                Solution::flatten(&mut node.right);
                // 叶子节点 or 无左子树
                if node.left.is_none() { return; }

                let mut right = node.right.take();
                node.right = node.left.take(); // 左子树放到右子树上

                // 遍历到右子树末尾，把剩余右子树接上
                let mut p = Rc::clone(node.right.as_ref().unwrap()); 
                while p.borrow().right.is_some() {
                    let next_p = Rc::clone(p.borrow().right.as_ref().unwrap());
                    p = next_p;
                }
                p.borrow_mut().right = right.take();
            },
            None => {}
        }
    }
}


```