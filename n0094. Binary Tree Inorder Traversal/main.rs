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

