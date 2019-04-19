# Maximum Difference Between Node and Ancestor :star::star:
- 题目地址: [https://leetcode-cn.com/problems/maximum-difference-between-node-and-ancestor](https://leetcode-cn.com/problems/maximum-difference-between-node-and-ancestor)
- 执行时间: 12 ms 
- 内存消耗: 3.3 MB
- 通过日期: 2019-04-14 12:14

## 题目内容
<p>给定二叉树的根节点 <code>root</code>，找出存在于不同节点 <code>A</code> 和 <code>B</code> 之间的最大值 <code>V</code>，其中 <code>V = |A.val - B.val|</code>，且 <code>A</code> 是 <code>B</code> 的祖先。</p>

<p>（如果 A 的任何子节点之一为 B，或者 A 的任何子节点是 B 的祖先，那么我们认为 A 是 B 的祖先）</p>



<p><strong>示例：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/2whqcep.jpg" style="height: 230px; width: 300px;"></p>

<pre><strong>输入：</strong>[8,3,10,1,6,null,14,null,null,4,7,13]
<strong>输出：</strong>7
<strong>解释： </strong>
我们有大量的节点与其祖先的差值，其中一些如下：
|8 - 3| = 5
|3 - 7| = 4
|8 - 1| = 7
|10 - 13| = 3
在所有可能的差值中，最大值 7 由 |8 - 1| = 7 得出。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li>树中的节点数在 <code>2</code> 到 <code>5000</code> 之间。</li>
	<li>每个节点的值介于 <code>0</code> 到 <code>100000</code> 之间。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diff = 0;
        Solution::dfs(&root, &mut Vec::new(), &mut max_diff);
        max_diff
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ancestors: &mut Vec<i32>, max_diff: &mut i32) {
        match root {
            None => { return; }
            Some(node) => {
                let node = node.borrow();
                for ancestor in ancestors.iter() { // 根据祖先列表求出最大值
                    *max_diff = (ancestor - node.val).abs().max(*max_diff);
                }
                ancestors.push(node.val);
                Solution::dfs(&node.left, ancestors, max_diff);
                Solution::dfs(&node.right, ancestors, max_diff);
                ancestors.pop();
            }
        }
    }
}


```