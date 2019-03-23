### House Robber III :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/house-robber-iii](https://leetcode-cn.com/problems/house-robber-iii)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 3.4 MB
- 通过日期/Accept Datetime: 2019-03-23 17:22

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rob_(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(node) => {
                    let node = node.borrow();
                    let (left_rob, left_not_rob) = rob_(&node.left);
                    let (right_rob, right_not_rob) = rob_(&node.right);
                    (node.val + left_not_rob + right_not_rob,                // rob this node
                     left_rob.max(left_not_rob) + right_rob.max(right_not_rob)) // not rob this node
                },
                None => {
                    (0, 0)
                }
            }
        }

        let (root_rob, root_not_rob) = rob_(&root);
        root_rob.max(root_not_rob)
    }
}


```
