// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let val = preorder[0];
        let lvals = preorder.iter().skip(1).filter(|&x| { *x < val }).cloned().collect();
        let rvals = preorder.iter().skip(1).filter(|&x| { *x > val }).cloned().collect();
        let mut node = TreeNode::new(val);
        node.left = Solution::bst_from_preorder(lvals);
        node.right = Solution::bst_from_preorder(rvals);

        Some(Rc::new(RefCell::new(node)))
    }
}

