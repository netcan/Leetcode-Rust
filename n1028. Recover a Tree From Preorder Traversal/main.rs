// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<i32> = s.split('-').filter(|x| x.len() > 0).map(|x| x.parse().unwrap()).collect();
        let mut depths: Vec<i32> = s.split(char::is_numeric).filter(|x| x.len() > 0).map(|x| x.len() as i32).collect();
        depths.insert(0, 0); // 根节点

        Solution::build_tree(&vals, &depths, 0, 0).0
    }

    fn build_tree(vals: &Vec<i32>, depths: &Vec<i32>, idx: usize, depth: usize) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if idx >= vals.len() || depth as i32 > depths[idx]  {
            return (None, idx);
        }
        let child = Some(Rc::new(RefCell::new(TreeNode::new(vals[idx]))));
        let (left, left_idx) = Solution::build_tree(vals, depths, idx + 1, depth + 1);
        let mut next_idx = left_idx;
        child.as_ref().unwrap().borrow_mut().left = left;
        if left_idx > idx { // 有右子树
            let (right, right_idx) = Solution::build_tree(vals, depths, left_idx, depth + 1);
            next_idx = right_idx;
            child.as_ref().unwrap().borrow_mut().right = right;
        }

        (child, next_idx)
    }
}

