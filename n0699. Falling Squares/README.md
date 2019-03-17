### Falling Squares :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/falling-squares](https://leetcode-cn.com/problems/falling-squares)
- 执行时间/Runtime: 24 ms 
- 内存消耗/Mem Usage: 2.8 MB
- 通过日期/Accept Datetime: 2019-03-16 17:12

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

struct Box {
    l: i32,
    r: i32,
    h: i32,
}

impl Solution {
    pub fn falling_squares(mut positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_height = 0;
        let mut height = vec![];
        let mut boxes: Vec<Box> = vec![];
        for p in &positions {
            let (cl, cr) = (p[0], p[0] + p[1]);
            let mut maxh = p[1];
            for b in &boxes {
                // 计算重叠部分
                if cr > b.l && cl < b.r { maxh = maxh.max(b.h + p[1]); }
            }
            boxes.push(Box {
                l: cl,
                r: cr,
                h: maxh // 更新最大值
            });
            max_height = max_height.max(maxh);
            height.push(max_height);
        }
        height
    }
}


```
