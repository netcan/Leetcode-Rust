
### Binary Tree Preorder Traversal
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-tree-preorder-traversal](https://leetcode-cn.com/problems/binary-tree-preorder-traversal)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 712.7 KB
- 提交日期/Datime: 2019-02-19 20:46

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        match root {
            None => {},
            Some(root) => {
                ret.push(root.borrow().val);
                ret.append(&mut Self::preorder_traversal(root.borrow().left.clone()));
                ret.append(&mut Self::preorder_traversal(root.borrow().right.clone()));
            }
        }
        ret
    }
}

```
