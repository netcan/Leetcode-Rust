## Construct Binary Search Tree from Preorder Traversal :star::star:
- 题目地址: [https://leetcode-cn.com/problems/construct-binary-search-tree-from-preorder-traversal](https://leetcode-cn.com/problems/construct-binary-search-tree-from-preorder-traversal)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-12 15:29

### 题目内容
---
<p>返回与给定先序遍历 <code>preorder</code> 相匹配的二叉搜索树（binary <strong>search</strong> tree）的根结点。</p>

<p><em>(回想一下，二叉搜索树是二叉树的一种，其每个节点都满足以下规则，对于 <code>node.left</code> 的任何后代，值总 <code><</code> <code>node.val</code>，而 <code>node.right</code> 的任何后代，值总 <code>></code> <code>node.val</code>。此外，先序遍历首先显示节点的值，然后遍历 <code>node.left</code>，接着遍历 <code>node.right</code>。）</em></p>



<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>[8,5,1,7,10,12]
<strong>输出：</strong>[8,5,10,1,7,null,12]
<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/03/08/1266.png" style="height: 200px; width: 306px;">
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= preorder.length <= 100</code></li>
	<li>先序 <code>preorder</code> 中的值是不同的。</li>
</ol>


### 解法
---
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