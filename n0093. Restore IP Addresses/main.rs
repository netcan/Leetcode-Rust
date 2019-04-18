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

