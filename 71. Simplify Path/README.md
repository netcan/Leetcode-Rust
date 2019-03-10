
### Simplify Path :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/simplify-path](https://leetcode-cn.com/problems/simplify-path)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 提交日期/Datetime: 2019-03-05 19:51

```rust
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let dir: Vec<&str> = path.split('/').collect();
        let mut dir_abs = Vec::new();
        for d in &dir {
            if d.len() == 0 || *d == "." {
                continue;
            } else if *d == ".." {
                dir_abs.pop();
                continue;
            }
            dir_abs.push(*d);
        }

        let ret = "/".to_owned() + &dir_abs.join("/");
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_1() {
        assert_eq!(Solution::simplify_path("/asdf/./sdf/".to_string()), "/asdf/sdf");
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo");
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c");
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::simplify_path("/a/../../b/../c//.//".to_string()), "/c");
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::simplify_path("/a//b////c/d//././/..".to_string()), "/a/b/c");
    }
}


```
