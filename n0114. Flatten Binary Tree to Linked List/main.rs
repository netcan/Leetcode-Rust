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

