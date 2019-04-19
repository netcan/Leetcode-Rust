# Sum of Root To Leaf Binary Numbers :star:
- 题目地址: [https://leetcode-cn.com/problems/sum-of-root-to-leaf-binary-numbers](https://leetcode-cn.com/problems/sum-of-root-to-leaf-binary-numbers)
- 执行时间: 0 ms 
- 内存消耗: 2.6 MB
- 通过日期: 2019-04-07 12:18

## 题目内容
<p>给出一棵二叉树，其上每个结点的值都是 <code>0</code> 或 <code>1</code> 。每一条从根到叶的路径都代表一个从最高有效位开始的二进制数。例如，如果路径为 <code>0 -> 1 -> 1 -> 0 -> 1</code>，那么它表示二进制数 <code>01101</code>，也就是 <code>13</code> 。</p>

<p>对树上的每一片叶子，我们都要找出从根到该叶子的路径所表示的数字。</p>

<p>以<strong> <code>10^9 + 7</code></strong> 为<strong>模</strong>，返回这些数字之和。</p>



<p><strong>示例：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/05/sum-of-root-to-leaf-binary-numbers.png" style="height: 200px; width: 304px;"></p>

<pre><strong>输入：</strong>[1,0,1,0,1,0,1]
<strong>输出：</strong>22
<strong>解释：</strong>(100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li>树中的结点数介于 <code>1</code> 和 <code>1000</code> 之间。</li>
	<li>node.val 为 <code>0</code> 或 <code>1</code> 。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    const Mod: i32 = 1000000007;
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 后序遍历
        Solution::post_order_sum(&root, 0)
    }
    fn post_order_sum(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        match root {
            None => { return 0; }
            Some(node) => {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() { // 叶子
                    return (val * 2 + node.val) % Solution::Mod;
                }
                let left_sum = Solution::post_order_sum(&node.left, (val * 2 + node.val) % Solution::Mod);
                let right_sum = Solution::post_order_sum(&node.right, (val * 2 + node.val) % Solution::Mod);
                return (left_sum + right_sum) % Solution::Mod;
            }
        }
    }
}

```