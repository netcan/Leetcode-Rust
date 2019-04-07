# Find Median from Data Stream :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/find-median-from-data-stream](https://leetcode-cn.com/problems/find-median-from-data-stream)
- 执行时间: 2056 ms 
- 内存消耗: 9.5 MB
- 通过日期: 2019-03-07 20:11

## 题目内容
<p>中位数是有序列表中间的数。如果列表长度是偶数，中位数则是中间两个数的平均值。</p>

<p>例如，</p>

<p>[2,3,4] 的中位数是 3</p>

<p>[2,3] 的中位数是 (2 + 3) / 2 = 2.5</p>

<p>设计一个支持以下两种操作的数据结构：</p>

<ul>
	<li>void addNum(int num) - 从数据流中添加一个整数到数据结构中。</li>
	<li>double findMedian() - 返回目前所有元素的中位数。</li>
</ul>

<p><strong>示例：</strong></p>

<pre>addNum(1)
addNum(2)
findMedian() -> 1.5
addNum(3) 
findMedian() -> 2</pre>

<p><strong>进阶:</strong></p>

<ol>
	<li>如果数据流中所有整数都在 0 到 100 范围内，你将如何优化你的算法？</li>
	<li>如果数据流中 99% 的整数都在 0 到 100 范围内，你将如何优化你的算法？</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::BTreeMap;

struct MedianFinder {
    container: BTreeMap<i32, i32>
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder{
            container: BTreeMap::new()
        }
    }

    fn add_num(&mut self, num: i32) {
        *self.container.entry(num).or_insert(0) += 1;
    }

    fn find_median(&self) -> f64 {
        let mut iter = self.container.iter();
        let n = self.container.values().sum::<i32>();

        let (mut pos, mid_pos) = (-1, ((n - 1)/ 2) as i32);
        let mut num = 0;

        while pos < mid_pos {
            let (k, v) = iter.next().unwrap();
            let times = *v;
            num = *k;
            if pos + times > mid_pos {
                return num as f64;
            }
            pos += times;
        }

        if n % 2 == 0 {
            (num + *iter.next().unwrap().0) as f64 / 2.0
        } else {
            num as f64
        }
    }
}


```