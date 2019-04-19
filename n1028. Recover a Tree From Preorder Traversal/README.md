# Recover a Tree From Preorder Traversal :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal](https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal)
- 执行时间: 4 ms 
- 内存消耗: 2.7 MB
- 通过日期: 2019-04-19 20:09

## 题目内容
<p>我们从二叉树的根节点 <code>root</code> 开始进行深度优先搜索。</p>

<p>在遍历中的每个节点处，我们输出 <code>D</code> 条短划线（其中 <code>D</code> 是该节点的深度），然后输出该节点的值。（<em>如果节点的深度为 <code>D</code>，则其直接子节点的深度为 <code>D + 1</code>。根节点的深度为 <code>0</code>）。</em></p>

<p>如果节点只有一个子节点，那么保证该子节点为左子节点。</p>

<p>给出遍历输出 <code>S</code>，还原树并返回其根节点 <code>root</code>。</p>



<p><strong>示例 1：</strong></p>

<p><strong><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/recover-a-tree-from-preorder-traversal.png" style="height: 200px; width: 320px;"></strong></p>

<pre><strong>输入：</strong>"1-2--3--4-5--6--7"
<strong>输出：</strong>[1,2,5,3,4,6,7]
</pre>

<p><strong>示例 2：</strong></p>

<p><strong><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/screen-shot-2019-04-10-at-114101-pm.png" style="height: 250px; width: 256px;"></strong></p>

<pre><strong>输入：</strong>"1-2--3---4-5--6---7"
<strong>输出：</strong>[1,2,5,3,null,6,null,4,null,7]
</pre>

<p><strong>示例 3：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/screen-shot-2019-04-10-at-114955-pm.png" style="height: 250px; width: 276px;"></p>

<pre><strong>输入：</strong>"1-401--349---90--88"
<strong>输出：</strong>[1,401,null,349,88,90]
</pre>



<p><strong>提示：</strong></p>

<ul>
	<li>原始树中的节点数介于 <code>1</code> 和 <code>1000</code> 之间。</li>
	<li>每个节点的值介于 <code>1</code> 和 <code>10 ^ 9</code> 之间。</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<i32> = s.split('-').filter(|x| x.len() > 0).map(|x| x.parse().unwrap()).collect();
        let mut depths: Vec<i32> = s.split(char::is_numeric).filter(|x| x.len() > 0).map(|x| x.len() as i32).collect();
        depths.insert(0, 0); // 根节点

        Solution::build_tree(&vals, &depths, 0, 0).0
    }

    fn build_tree(vals: &Vec<i32>, depths: &Vec<i32>, idx: usize, depth: usize) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if idx >= vals.len() || depth as i32 > depths[idx]  {
            return (None, idx);
        }
        let child = Some(Rc::new(RefCell::new(TreeNode::new(vals[idx]))));
        let (left, left_idx) = Solution::build_tree(vals, depths, idx + 1, depth + 1);
        let mut next_idx = left_idx;
        child.as_ref().unwrap().borrow_mut().left = left;
        if left_idx > idx { // 有右子树
            let (right, right_idx) = Solution::build_tree(vals, depths, left_idx, depth + 1);
            next_idx = right_idx;
            child.as_ref().unwrap().borrow_mut().right = right;
        }

        (child, next_idx)
    }
}


```