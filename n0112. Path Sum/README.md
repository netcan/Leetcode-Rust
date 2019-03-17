### Path Sum :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/path-sum](https://leetcode-cn.com/problems/path-sum)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 3.2 MB
- 通过日期/Accept Datetime: 2019-03-10 00:04

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Solution::has_path_sum_(&root, sum)
    }

    fn has_path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                if let None = node.left {
                    if let None = node.right {
                        if diff == 0 { return true; }
                    }
                }
                Solution::has_path_sum_(&node.left, diff) ||
                Solution::has_path_sum_(&node.right, diff)
            },
            None => false
        }
    }

}

```
