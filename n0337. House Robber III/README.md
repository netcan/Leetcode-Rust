# House Robber III :star::star:
- 题目地址: [https://leetcode-cn.com/problems/house-robber-iii](https://leetcode-cn.com/problems/house-robber-iii)
- 执行时间: 4 ms 
- 内存消耗: 3.4 MB
- 通过日期: 2019-03-23 17:22

## 题目内容
<p>在上次打劫完一条街道之后和一圈房屋后，小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为“根”。 除了“根”之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果两个直接相连的房子在同一天晚上被打劫，房屋将自动报警。</p>

<p>计算在不触动警报的情况下，小偷一晚能够盗取的最高金额。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入: </strong>[3,2,3,null,3,null,1]

     <strong>3</strong>
    / \
   2   3
    \   \ 
     <strong>3</strong>   <strong>1</strong>

<strong>输出:</strong> 7 
<strong>解释:</strong> 小偷一晚能够盗取的最高金额 = 3 + 3 + 1 = <strong>7</strong>.</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong>[3,4,5,1,3,null,1]

     3
    / \
   <strong>4</strong>   <strong>5</strong>
  / \   \ 
 1   3   1

<strong>输出:</strong> 9
<strong>解释:</strong> 小偷一晚能够盗取的最高金额 = <strong>4</strong> + <strong>5</strong> = <strong>9</strong>.
</pre>


## 解法
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