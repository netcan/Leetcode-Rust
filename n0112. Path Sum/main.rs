// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Solution::has_path_sum_(&root, sum)
    }

    fn has_path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                if let None = node.left {
                    if let None = node.right {
                        if diff == 0 { return true; }
                    }
                }
                Solution::has_path_sum_(&node.left, diff) ||
                Solution::has_path_sum_(&node.right, diff)
            },
            None => false
        }
    }

}
