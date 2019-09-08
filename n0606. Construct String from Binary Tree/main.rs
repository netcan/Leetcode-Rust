// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn tree2str_(tree: &Option<Rc<RefCell<TreeNode>>>) -> String {
            match tree {
                None => format!(""),
                Some(root) => {
                    let node = root.borrow();
                    let (left_str, right_str) =
                        (tree2str_(&node.left), tree2str_(&node.right));
                    if right_str.len() > 0 { format!("{}({})({})", node.val, left_str, right_str) } 
                    else if left_str.len() > 0{ format!("{}({})", node.val, left_str) } 
                    else { format!("{}", node.val) }
                }
            }
        }
        tree2str_(&t)
    }
}

