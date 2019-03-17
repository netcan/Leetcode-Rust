### Binary Tree Inorder Traversal :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-tree-inorder-traversal](https://leetcode-cn.com/problems/binary-tree-inorder-traversal)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 716.8 KB
- 通过日期/Accept Datetime: 2019-02-19 20:40

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        if let Some(root) = root {
            if let Some(left) = &(*root.borrow()).left {
                ret.append(&mut Self::inorder_traversal(Some(Rc::clone(left))));
            }
            ret.push((*root.borrow()).val);
            if let Some(right) = &(*root.borrow()).right {
                ret.append(&mut Self::inorder_traversal(Some(Rc::clone(right))));
            }
        }
        ret
    }
}


```
