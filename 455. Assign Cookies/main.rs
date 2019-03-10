impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut ret = 0;
        let mut j = 0;
        for ss in &s {
            if j >= g.len() {
                break;
            }
            if *ss >= g[j] {
                ret += 1;
                j += 1;
            }
        }

        ret
    }
}

