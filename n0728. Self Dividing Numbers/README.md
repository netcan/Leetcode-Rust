## Self Dividing Numbers :star:
- 题目地址: [https://leetcode-cn.com/problems/self-dividing-numbers](https://leetcode-cn.com/problems/self-dividing-numbers)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-06 09:28

### 题目内容
<p><em>自除数 </em>是指可以被它包含的每一位数除尽的数。</p>

<p>例如，128 是一个自除数，因为 <code>128 % 1 == 0</code>，<code>128 % 2 == 0</code>，<code>128 % 8 == 0</code>。</p>

<p>还有，自除数不允许包含 0 。</p>

<p>给定上边界和下边界数字，输出一个列表，列表的元素是边界（含边界）内所有的自除数。</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong> 
上边界left = 1, 下边界right = 22
<strong>输出：</strong> [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
</pre>

<p><strong>注意：</strong></p>

<ul>
	<li>每个输入参数的边界满足 <code>1 <= left <= right <= 10000</code>。</li>
</ul>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let ret = (left..=right).filter(|&x| {
            let mut _x = x;
            let mut dividing = true;
            while(_x > 0) {
                if(_x % 10 == 0 || (x % (_x % 10)) != 0) {
                    dividing = false;
                    break;
                }
                _x /= 10;
            }
            dividing
        }).collect();
        ret
    }
}


```
