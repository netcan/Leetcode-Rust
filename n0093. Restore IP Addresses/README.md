# Restore IP Addresses :star::star:
- 题目地址: [https://leetcode-cn.com/problems/restore-ip-addresses](https://leetcode-cn.com/problems/restore-ip-addresses)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-18 21:37

## 题目内容
<p>给定一个只包含数字的字符串，复原它并返回所有可能的 IP 地址格式。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> "25525511135"
<strong>输出:</strong> <code>["255.255.11.135", "255.255.111.35"]</code></pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s: Vec<i32> = s.into_bytes().into_iter().map(|x| (x - '0' as u8) as i32).collect();
        let mut ip_addrs: Vec<String> = vec![];

        Solution::restore_ip_addresses_(&s, s.len(), &mut Vec::new(), &mut ip_addrs);
        ip_addrs
    }
    pub fn restore_ip_addresses_(s: &[i32], s_len: usize, ip: &mut Vec<i32>, ip_addrs: &mut Vec<String>) {
        if ip.len() > 4 { return; }
        else if ip.len() == 4 && s.len() == 0 {
            let ip_addr = ip.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
            if ip_addr.len() - 3 == s_len { ip_addrs.push(ip_addr); }
        }
        let mut num = 0;
        for i in 0..s.len() {
            num = num * 10 + s[i];
            if num < 256 {
                ip.push(num);
                Solution::restore_ip_addresses_(&s[i + 1..], s_len, ip, ip_addrs);
                ip.pop();
            } else { break; }
        }

    }
}


```