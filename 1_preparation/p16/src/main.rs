use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, a: [i64; n]) -> i64 {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(a[i]);
    }
    v.sort();
    v.reverse();
    for i in 0..n-2 {
        if v[i] < v[i+1] + v[i+2] {
            return v[i] + v[i+1] + v[i+2];
        }
    }
    0
}
