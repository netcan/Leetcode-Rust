// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<Vec<u8>> = nums.into_iter().map(|x| x.to_string().into_bytes()).collect();
        nums.sort_by(|a, b| {
            let mut i = 0;
            while i < a.len() || i < b.len() {
                let (ac_end, bc_end) = ( // 当前数字 or 结尾数字
                    a[i.min(a.len() - 1)],
                    b[i.min(b.len() - 1)]
                    );

                let (ac_begin, bc_begin) = ( // 当前数字 or 开头数字
                    a[i.min(a.len()) % a.len()],
                    b[i.min(b.len()) % b.len()]
                    );

                let (ac, bc) = (
                    ac_begin.max(ac_end), 
                    bc_begin.max(bc_end)
                );

                if ac < bc { return Ordering::Greater; }
                else if ac > bc { return Ordering::Less; }
                else { // ac == bc
                    if ac_end < bc_end { return Ordering::Greater; }
                    else if ac_end > bc_end { return Ordering::Less; }
                }
                i += 1;
            }
            Ordering::Equal
        });

        if nums.iter().flatten().all(|x| *x == '0' as u8) { return "0".to_string(); }
        String::from_utf8(nums.into_iter().flatten().collect()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_number(vec![96, 969]), "96996".to_owned());
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_number(vec![12, 121]), "12121".to_owned());
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_number(vec![824,938,1399,5607,6973,5703,9609,4398,8247]), "9609938824824769735703560743981399".to_owned());
    }

}

