
### Binary Tree Postorder Traversal
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-tree-postorder-traversal](https://leetcode-cn.com/problems/binary-tree-postorder-traversal)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 708.6 KB
- 提交日期/Datime: 2019-02-19 20:47

```rust
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
