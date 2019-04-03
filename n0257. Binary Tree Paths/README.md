## Binary Tree Paths :star:
- 题目地址: [https://leetcode-cn.com/problems/binary-tree-paths](https://leetcode-cn.com/problems/binary-tree-paths)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-06 23:17

### 题目内容
<p>给定一个二叉树，返回所有从根节点到叶子节点的路径。</p>

<p><strong>说明:</strong> 叶子节点是指没有子节点的节点。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong>

   1
 /   \
2     3
 \
  5

<strong>输出:</strong> ["1->2->5", "1->3"]

<strong>解释:</strong> 所有根节点到叶子节点的路径为: 1->2->5, 1->3</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

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



```
