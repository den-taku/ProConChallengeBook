use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, m: i64, a: [i64; n]) -> bool {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    if a[i] + a[j] + a[k] + a[l] == m {
                        return true;
                    }
                }
            }
        }
    }
    false
}
