// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ret = vec![];
        Solution::binary_tree(&root, &mut ret, &String::new());
        ret
    }

    fn binary_tree(root: &Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, path: &String) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let mut path_tmp = path.clone();
                if path_tmp.len() != 0 {
                    path_tmp.push_str("->");
                }
                path_tmp.push_str(&node.val.to_string());
                if let None = node.left {
                    if let None = node.right {
                        paths.push(path_tmp.clone());
                    }
                }
                Solution::binary_tree(&node.left, paths, &path_tmp);
                Solution::binary_tree(&node.right, paths, &path_tmp);
            }
            _ => {}
        }
    }
}

