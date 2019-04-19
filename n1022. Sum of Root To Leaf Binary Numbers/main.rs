// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    const Mod: i32 = 1000000007;
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 后序遍历
        Solution::post_order_sum(&root, 0)
    }
    fn post_order_sum(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        match root {
            None => { return 0; }
            Some(node) => {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() { // 叶子
                    return (val * 2 + node.val) % Solution::Mod;
                }
                let left_sum = Solution::post_order_sum(&node.left, (val * 2 + node.val) % Solution::Mod);
                let right_sum = Solution::post_order_sum(&node.right, (val * 2 + node.val) % Solution::Mod);
                return (left_sum + right_sum) % Solution::Mod;
            }
        }
    }
}
