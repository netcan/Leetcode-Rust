## Compare Version Numbers :star::star:
- 题目地址: [https://leetcode-cn.com/problems/compare-version-numbers](https://leetcode-cn.com/problems/compare-version-numbers)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-21 15:14

### 题目内容
<p>比较两个版本号 <em>version1 </em>和 <em>version2</em>。<br>
如果 <code><em>version1 </em>> <em>version2</em></code> 返回 <code>1</code>，如果 <code><em>version1 </em>< <em>version2</em></code> 返回 <code>-1</code>， 除此之外返回 <code>0</code>。</p>

<p>你可以假设版本字符串非空，并且只包含数字和 <code>.</code> 字符。</p>

<p> <code>.</code> 字符不代表小数点，而是用于分隔数字序列。</p>

<p>例如，<code>2.5</code> 不是“两个半”，也不是“差一半到三”，而是第二版中的第五个小版本。</p>

<p>你可以假设版本号的每一级的默认修订版号为 <code>0</code>。例如，版本号 <code>3.4</code> 的第一级（大版本）和第二级（小版本）修订号分别为 <code>3</code> 和 <code>4</code>。其第三级和第四级修订号均为 <code>0</code>。<br>
 </p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> <code><em>version1</em></code> = "0.1", <code><em>version2</em></code> = "1.1"
<strong>输出:</strong> -1</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong><code><em>version1</em></code> = "1.0.1", <code><em>version2</em></code> = "1"
<strong>输出:</strong> 1</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入:</strong> <code><em>version1</em></code> = "7.5.2.4", <code><em>version2</em></code> = "7.5.3"
<strong>输出:</strong> -1</pre>

<p><strong>示例 4：</strong></p>

<pre><code><strong>输入：</strong><em>version1</em></code> = "1.01", <code><em>version2</em></code> = "1.001"
<strong>输出：</strong>0
<strong>解释：</strong>忽略前导零，“01” 和 “001” 表示相同的数字 “1”。</pre>

<p><strong>示例 5：</strong></p>

<pre><code><strong>输入：</strong><em>version1</em></code> = "1.0", <code><em>version2</em></code> = "1.0.0"
<strong>输出：</strong>0
<strong>解释：</strong><code><em>version1 </em></code>没有第三级修订号，这意味着它的第三级修订号默认为 “0”。</pre>



<p><strong>提示：</strong></p>

<ol>
	<li>版本字符串由以点 （<code>.</code>） 分隔的数字字符串组成。这个数字字符串<strong>可能</strong>有前导零。</li>
	<li>版本字符串不以点开始或结束，并且其中不会有两个连续的点。</li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        if version1.len() <= 0 || version2.len() <= 0 { return 0; }

        let version1: Vec<&str> = version1.split('.').collect();
        let version2: Vec<&str> = version2.split('.').collect();
        let mut i = 0;
        while i < version1.len() || i < version2.len() {
            let num1: u32 = if i < version1.len() {
                version1[i].parse().unwrap()
            } else { 0 };

            let num2: u32 = if i < version2.len() {
                version2[i].parse().unwrap()
            } else { 0 };
            if num1 > num2 { return 1; }
            else if num1 < num2 { return -1; }
            i += 1;
        }

        0
    }
}


```
