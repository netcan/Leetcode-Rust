impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(a.len());
        unsafe { ret.set_len(a.len()); }
        let (mut i, mut j) = (0, 1);
        for n in &a {
            if n & 1 == 1 {
                ret[j] = *n;
                j += 2;
            } else {
                ret[i] = *n;
                i += 2;
            }
        }
        ret
    }
}
