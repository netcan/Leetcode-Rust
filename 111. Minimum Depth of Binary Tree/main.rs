use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let None = root {
            return 0;
        }
        let mut que: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
        que.push_back((root, 1));
        while !que.is_empty() {
            let (node, depth) = que.pop_front().unwrap();
            match node {
                Some(node) => {
                    let node = node.borrow();
                    if let None = node.left {
                        if let None = node.right {
                            return depth;
                        }
                    }

                    if let Some(_) = node.left {
                        que.push_back((node.left.clone(), depth + 1));
                    }
                    
                    if let Some(_) = node.right {
                        que.push_back((node.right.clone(), depth + 1));
                    }

                },
                None => {
                    return depth;
                }
            }

        }
        0

    }
}
