# Construct String from Binary Tree :star:
- 题目地址: [https://leetcode-cn.com/problems/construct-string-from-binary-tree](https://leetcode-cn.com/problems/construct-string-from-binary-tree)
- 执行时间: 12 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-25 17:25

## 题目内容
<p>你需要采用前序遍历的方式，将一个二叉树转换成一个由括号和整数组成的字符串。</p>

<p>空节点则用一对空括号 "()" 表示。而且你需要省略所有不影响字符串与原始二叉树之间的一对一映射关系的空括号对。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> 二叉树: [1,2,3,4]
       1
     /   \
    2     3
   /    
  4     

<strong>输出:</strong> "1(2(4))(3)"

<strong>解释:</strong> 原本将是“1(2(4)())(3())”，
在你省略所有不必要的空括号对之后，
它将是“1(2(4))(3)”。
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> 二叉树: [1,2,3,null,4]
       1
     /   \
    2     3
     \  
      4 

<strong>输出:</strong> "1(2()(4))(3)"

<strong>解释:</strong> 和第一个示例相似，
除了我们不能省略第一个对括号来中断输入和输出之间的一对一映射关系。
</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn tree2str_(tree: &Option<Rc<RefCell<TreeNode>>>) -> String {
            match tree {
                None => format!(""),
                Some(root) => {
                    let node = root.borrow();
                    let (left_str, right_str) =
                        (tree2str_(&node.left), tree2str_(&node.right));
                    if right_str.len() > 0 { format!("{}({})({})", node.val, left_str, right_str) } 
                    else if left_str.len() > 0{ format!("{}({})", node.val, left_str) } 
                    else { format!("{}", node.val) }
                }
            }
        }
        tree2str_(&t)
    }
}


```