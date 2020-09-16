use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, s: [usize; n], t: [usize; n]) -> i32 {
    let mut v = Vec::new();
    for i in 0..n {
        v.push((s[i], t[i]));
    }
    v.sort_by_key(|s| s.1);
    let mut ans = 0;
    let mut t = 0;
    for i in 0..n {
        if t < v[i].0 {
            ans += 1;
            t = v[i].1;
        }
    }
    ans
}
