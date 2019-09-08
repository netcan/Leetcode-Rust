# Binary Watch :star:
- 题目地址: [https://leetcode-cn.com/problems/binary-watch](https://leetcode-cn.com/problems/binary-watch)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-20 16:47

## 题目内容
<p>二进制手表顶部有 4 个 LED 代表<strong>小时（0-11）</strong>，底部的 6 个 LED 代表<strong>分钟（0-59）</strong>。</p>

<p>每个 LED 代表一个 0 或 1，最低位在右侧。</p>

<p><img src="https://upload.wikimedia.org/wikipedia/commons/8/8b/Binary_clock_samui_moon.jpg" style="height:300px" /></p>

<p>例如，上面的二进制手表读取 “3:25”。</p>

<p>给定一个非负整数 <em>n </em>代表当前 LED 亮着的数量，返回所有可能的时间。</p>

<p><strong>案例:</strong></p>

<pre>
输入: n = 1
返回: ["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]</pre>



<p><strong>注意事项:</strong></p>

<ul>
	<li>输出的顺序没有要求。</li>
	<li>小时不会以零开头，比如 “01:00” 是不允许的，应为 “1:00”。</li>
	<li>分钟必须由两位数组成，可能会以零开头，比如 “10:2” 是无效的，应为 “10:02”。</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut watch = vec![];
        for led in 0..1<<10 {
            let (mut led_num, mut led_) = (0, led);
            while led_ > 0 {
                led_ &= led_ - 1;
                led_num += 1;
            }

            if led_num == num {
                let (hour, min) = (led >> 6, led & 0x3f);
                if hour < 12 && min < 60 {
                    watch.push(format!("{}:{:02}", led >> 6, led & 0x3f))
                }
            }
        }

        watch
    }
}


```