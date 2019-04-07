# Knight Dialer :star::star:
- 题目地址: [https://leetcode-cn.com/problems/knight-dialer](https://leetcode-cn.com/problems/knight-dialer)
- 执行时间: 16 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-06 18:27

## 题目内容
<p>国际象棋中的骑士可以按下图所示进行移动：</p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/11/03/knight.png" style="height: 150px; width: 150px;"> .           <img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/11/03/keypad.png" style="height: 150px; width: 134px;"></p>

<p><br>
这一次，我们将 “骑士” 放在电话拨号盘的任意数字键（如上图所示）上，接下来，骑士将会跳 N-1 步。每一步必须是从一个数字键跳到另一个数字键。</p>

<p>每当它落在一个键上（包括骑士的初始位置），都会拨出键所对应的数字，总共按下 <code>N</code> 位数字。</p>

<p>你能用这种方式拨出多少个不同的号码？</p>

<p>因为答案可能很大，<strong>所以输出答案模 <code>10^9 + 7</code></strong>。</p>



<ul>
</ul>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>1
<strong>输出：</strong>10
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>2
<strong>输出：</strong>20
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>3
<strong>输出：</strong>46
</pre>



<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= N <= 5000</code></li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let n: usize = n as usize;
        let NEXT_STEP = vec![
            vec![4, 6],    // 0
            vec![6, 8],    // 1
            vec![7, 9],    // 2
            vec![4, 8],    // 3
            vec![0, 3, 9], // 4
            vec![],        // 5
            vec![0, 1, 7], // 6
            vec![2, 6],    // 7
            vec![1, 3],    // 8
            vec![4, 2],    // 9
        ];
        let mut dp = [[0; 10], [1; 10]];
        for s in 2..=n {
            for d in 0..=9 {
                dp[s & 1][d] = 0;
                for ns in &NEXT_STEP[d] {
                    dp[s & 1][d] += dp[(s+1) & 1][*ns];
                    dp[s & 1][d] %= 1000000007;
                }
            }
        }
        // println!("{:#?}", dp);
        
        dp[n & 1].iter().fold(0, |a, &b| { (a+b) % 1000000007 })
    }
}


```