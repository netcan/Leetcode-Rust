## Path Sum II :star::star:
- 题目地址: [https://leetcode-cn.com/problems/path-sum-ii](https://leetcode-cn.com/problems/path-sum-ii)
- 执行时间: 4 ms 
- 内存消耗: 6.7 MB
- 通过日期: 2019-03-10 00:48

### 题目内容
<p>给定一个二叉树和一个目标和，找到所有从根节点到叶子节点路径总和等于给定目标和的路径。</p>

<p><strong>说明:</strong> 叶子节点是指没有子节点的节点。</p>

<p><strong>示例:</strong><br>
给定如下二叉树，以及目标和 <code>sum = 22</code>，</p>

<pre>              <strong>5</strong>
             / \
            <strong>4</strong>   <strong>8</strong>
           /   / \
          <strong>11</strong>  13  <strong>4</strong>
         /  \    / \
        7    <strong>2</strong>  <strong>5</strong>   1
</pre>

<p>返回:</p>

<pre>[
   [5,4,11,2],
   [5,8,4,5]
]
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Solution::path_sum_(&root, sum, vec![], &mut result);
        result
    }

    fn path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, mut path: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                path.push(node.val);
                if node.left.is_none() &&
                    node.right.is_none() &&
                        diff == 0 {
                            result.push(path.clone());
                        }
                Solution::path_sum_(&node.left, diff, path.clone(), result);
                Solution::path_sum_(&node.right, diff, path.clone(), result);
            },
            None => {}
        }
    }

}


```
