use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Solution::path_sum_(&root, sum, vec![], &mut result);
        result
    }

    fn path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, mut path: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                path.push(node.val);
                if node.left.is_none() &&
                    node.right.is_none() &&
                        diff == 0 {
                            result.push(path.clone());
                        }
                Solution::path_sum_(&node.left, diff, path.clone(), result);
                Solution::path_sum_(&node.right, diff, path.clone(), result);
            },
            None => {}
        }
    }

}
