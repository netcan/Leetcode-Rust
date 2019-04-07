# Path Sum III :star:
- 题目地址: [https://leetcode-cn.com/problems/path-sum-iii](https://leetcode-cn.com/problems/path-sum-iii)
- 执行时间: 20 ms 
- 内存消耗: 2.8 MB
- 通过日期: 2019-03-10 00:38

## 题目内容
<p>给定一个二叉树，它的每个结点都存放着一个整数值。</p>

<p>找出路径和等于给定数值的路径总数。</p>

<p>路径不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的（只能从父节点到子节点）。</p>

<p>二叉树不超过1000个节点，且节点数值范围是 [-1000000,1000000] 的整数。</p>

<p><strong>示例：</strong></p>

<pre>root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8

      10
     /  \
    <strong>5</strong>   <strong>-3</strong>
   <strong>/</strong> <strong>\</strong>    <strong>\</strong>
  <strong>3</strong>   <strong>2</strong>   <strong>11</strong>
 / \   <strong>\</strong>
3  -2   <strong>1</strong>

返回 3。和等于 8 的路径有:

1.  5 -> 3
2.  5 -> 2 -> 1
3.  -3 -> 11
</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut cnt = 0;
        Solution::traverse(&root, sum, &mut cnt);
        cnt
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, count: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            Solution::path_sum_(&root, sum, count);
            Solution::traverse(&node.left, sum, count);
            Solution::traverse(&node.right, sum, count);
        }
    }

    fn path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, count: &mut i32) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let diff = sum - node.val;
                if diff == 0 { *count += 1; }
                Solution::path_sum_(&node.left, diff, count);
                Solution::path_sum_(&node.right, diff, count);
            },
            None => {}
        }
    }

}


```