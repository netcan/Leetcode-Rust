### Binary Tree Tilt :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-tree-tilt](https://leetcode-cn.com/problems/binary-tree-tilt)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 3.2 MB
- 通过日期/Accept Datetime: 2019-03-11 11:54

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tilt = 0;
        Solution::cal_tilt_(&root, &mut tilt);
        tilt
    }
    fn cal_tilt_(root: &Option<Rc<RefCell<TreeNode>>>, tilt: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();

                let lsum = Solution::cal_tilt_(&node.left, tilt);
                let rsum = Solution::cal_tilt_(&node.right, tilt);
                *tilt += (lsum - rsum).abs();
                lsum + rsum + node.val
            },
            None => 0
        }

    }
}


```
