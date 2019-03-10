use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        match root {
            None => {},
            Some(root) => {
                ret.push(root.borrow().val);
                ret.append(&mut Self::preorder_traversal(root.borrow().left.clone()));
                ret.append(&mut Self::preorder_traversal(root.borrow().right.clone()));
            }
        }
        ret
    }
}
