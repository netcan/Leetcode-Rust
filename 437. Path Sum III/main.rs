use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut cnt = 0;
        Solution::traverse(&root, sum, &mut cnt);
        cnt
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, count: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            Solution::path_sum_(&root, sum, count);
            Solution::traverse(&node.left, sum, count);
            Solution::traverse(&node.right, sum, count);
        }
    }

    fn path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, count: &mut i32) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                if diff == 0 { *count += 1; }
                Solution::path_sum_(&node.left, diff, count);
                Solution::path_sum_(&node.right, diff, count);
            },
            None => {}
        }
    }

}
