# Binary Tree Postorder Traversal :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/binary-tree-postorder-traversal](https://leetcode-cn.com/problems/binary-tree-postorder-traversal)
- 执行时间: 0 ms 
- 内存消耗: 708.6 KB
- 通过日期: 2019-02-19 20:47

## 题目内容
<p>给定一个二叉树，返回它的 <em>后序 </em>遍历。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> [1,null,2,3]  
   1
    \
     2
    /
   3 

<strong>输出:</strong> [3,2,1]</pre>

<p><strong>进阶:</strong> 递归算法很简单，你可以通过迭代算法完成吗？</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        match root {
            None => {},
            Some(root) => {
                ret.append(&mut Self::postorder_traversal(root.borrow().left.clone()));
                ret.append(&mut Self::postorder_traversal(root.borrow().right.clone()));
                ret.push(root.borrow().val);
            }
        }
        ret
    }
}


```