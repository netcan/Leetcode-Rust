### Construct Binary Search Tree from Preorder Traversal :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/construct-binary-search-tree-from-preorder-traversal](https://leetcode-cn.com/problems/construct-binary-search-tree-from-preorder-traversal)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-12 15:29

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let val = preorder[0];
        let lvals = preorder.iter().skip(1).filter(|&x| { *x < val }).cloned().collect();
        let rvals = preorder.iter().skip(1).filter(|&x| { *x > val }).cloned().collect();
        let mut node = TreeNode::new(val);
        node.left = Solution::bst_from_preorder(lvals);
        node.right = Solution::bst_from_preorder(rvals);

        Some(Rc::new(RefCell::new(node)))
    }
}


```
