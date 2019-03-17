### Binary Tree Maximum Path Sum :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-tree-maximum-path-sum](https://leetcode-cn.com/problems/binary-tree-maximum-path-sum)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 4.5 MB
- 通过日期/Accept Datetime: 2019-03-09 23:40

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::min_value();
        Solution::max_path_sum_(&root, &mut max_sum);
        max_sum
    }

    fn max_path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let mut lsum = Solution::max_path_sum_(&node.left, max_sum);
                if lsum == i32::min_value() {
                    lsum = 0;
                }
                let mut rsum = Solution::max_path_sum_(&node.right, max_sum);
                if rsum == i32::min_value() {
                    rsum = 0;
                }
                lsum += node.val;
                rsum += node.val;

                let lrv_max_sum = max(node.val, max(lsum, rsum));
                // println!("lsum: {} rsum: {} lrv:{} val: {}", lsum, rsum, lrv_max_sum, node.val);
                *max_sum = max(*max_sum, max(max(lrv_max_sum, lsum + rsum - node.val), node.val));

                lrv_max_sum
            },
            None => {
                i32::min_value()
            }
        }
    }

}

```
