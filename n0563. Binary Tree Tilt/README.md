## Binary Tree Tilt :star:
- 题目地址: [https://leetcode-cn.com/problems/binary-tree-tilt](https://leetcode-cn.com/problems/binary-tree-tilt)
- 执行时间: 4 ms 
- 内存消耗: 3.2 MB
- 通过日期: 2019-03-11 11:54

### 题目内容
<p>给定一个二叉树，计算<strong>整个树</strong>的坡度。</p>

<p>一个树的<strong>节点的坡度</strong>定义即为，该节点左子树的结点之和和右子树结点之和的<strong>差的绝对值</strong>。空结点的的坡度是0。</p>

<p><strong>整个树</strong>的坡度就是其所有节点的坡度之和。</p>

<p><strong>示例:</strong></p>

<pre>
<strong>输入:</strong> 
         1
       /   \
      2     3
<strong>输出:</strong> 1
<strong>解释:</strong> 
结点的坡度 2 : 0
结点的坡度 3 : 0
结点的坡度 1 : |2-3| = 1
树的坡度 : 0 + 0 + 1 = 1
</pre>

<p><strong>注意:</strong></p>

<ol>
	<li>任何子树的结点的和不会超过32位整数的范围。</li>
	<li>坡度的值不会超过32位整数的范围。</li>
</ol>


### 解法
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
