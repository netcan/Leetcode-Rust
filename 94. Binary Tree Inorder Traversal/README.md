
### Binary Tree Inorder Traversal :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-tree-inorder-traversal](https://leetcode-cn.com/problems/binary-tree-inorder-traversal)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 725 KB
- 提交日期/Datetime: 2019-02-19 20:44

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        match root {
            None => {},
            Some(root) => {
                ret.append(&mut Self::inorder_traversal(root.borrow().left.clone()));
                ret.push(root.borrow().val);
                ret.append(&mut Self::inorder_traversal(root.borrow().right.clone()));
            }
        }

        ret
    }
}


```
