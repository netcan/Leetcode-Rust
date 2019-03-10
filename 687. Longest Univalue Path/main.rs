use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_length = 0;
        Solution::longest_univalue_path_(&root, &mut max_length);
        max_length
    }

    fn longest_univalue_path_(root: &Option<Rc<RefCell<TreeNode>>>, max_length: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let l = Solution::longest_univalue_path_(&node.left, max_length);
                let r = Solution::longest_univalue_path_(&node.right, max_length);
                let (mut l_len, mut r_len) = (0, 0);
                if let Some(ref left) = node.left {
                    if left.borrow().val == node.val {
                        l_len = l + 1;
                    }
                }
                if let Some(ref right) = node.right {
                    if right.borrow().val == node.val {
                        r_len = r + 1;
                    }
                }
                *max_length = max(*max_length, l_len + r_len);
                max(l_len, r_len)
            },
            None => {
                0
            }
        }
    }
}