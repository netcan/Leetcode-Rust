# Capacity To Ship Packages Within D Days :star::star:
- 题目地址: [https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days](https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days)
- 执行时间: 16 ms 
- 内存消耗: 3.3 MB
- 通过日期: 2019-03-17 16:26

## 题目内容
<p>传送带上的包裹必须在 D 天内从一个港口运送到另一个港口。</p>

<p>传送带上的第 <code>i</code> 个包裹的重量为 <code>weights[i]</code>。每一天，我们都会按给出重量的顺序往传送带上装载包裹。我们装载的重量不会超过船的最大运载重量。</p>

<p>返回能在 <code>D</code> 天内将传送带上的所有包裹送达的船的最低运载能力。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>weights = [1,2,3,4,5,6,7,8,9,10], D = 5
<strong>输出：</strong>15
<strong>解释：</strong>
船舶最低载重 15 就能够在 5 天内送达所有包裹，如下所示：
第 1 天：1, 2, 3, 4, 5
第 2 天：6, 7
第 3 天：8
第 4 天：9
第 5 天：10

请注意，货物必须按照给定的顺序装运，因此使用载重能力为 14 的船舶并将包装分成 (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) 是不允许的。 
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>weights = [3,2,2,4,1,4], D = 3
<strong>输出：</strong>6
<strong>解释：</strong>
船舶最低载重 6 就能够在 3 天内送达所有包裹，如下所示：
第 1 天：3, 2
第 2 天：2, 4
第 3 天：1, 4
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>weights = [1,2,3,1,1], D = 4
<strong>输出：</strong>3
<strong>解释：</strong>
第 1 天：1
第 2 天：2
第 3 天：3
第 4 天：1, 1
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= D <= weights.length <= 50000</code></li>
	<li><code>1 <= weights[i] <= 500</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let max_weight:i32 = *weights.iter().max().unwrap();
        let sum_weight:i32 = weights.iter().sum();

        let (mut lb, mut ub) = (max_weight, sum_weight + 1);
        while ub - lb > 1 {
            let mid = (lb + ub) / 2;
            if Solution::check_weight(&weights, mid, d) { ub = mid; }
            else { lb = mid; }
        }

        if Solution::check_weight(&weights, lb, d) { lb }
        else { ub }
    }

    fn check_weight(weights: &Vec<i32>, min_weight: i32, d: i32) -> bool {
        let mut curd = 0;
        let mut weight = 0;
        for w in weights {
            if weight + w > min_weight {
                weight = *w;
                curd += 1;
            } else {
                weight += w;
            }
            if curd >= d { return false; };
        }

        return true;
    }
}

```