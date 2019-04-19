// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diff = 0;
        Solution::dfs(&root, &mut Vec::new(), &mut max_diff);
        max_diff
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ancestors: &mut Vec<i32>, max_diff: &mut i32) {
        match root {
            None => { return; }
            Some(node) => {
                let node = node.borrow();
                for ancestor in ancestors.iter() { // 根据祖先列表求出最大值
                    *max_diff = (ancestor - node.val).abs().max(*max_diff);
                }
                ancestors.push(node.val);
                Solution::dfs(&node.left, ancestors, max_diff);
                Solution::dfs(&node.right, ancestors, max_diff);
                ancestors.pop();
            }
        }
    }
}

