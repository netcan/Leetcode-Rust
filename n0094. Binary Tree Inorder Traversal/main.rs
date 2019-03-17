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

