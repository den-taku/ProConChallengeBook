use competitive::prelude::*;
use std::cmp::max;

#[argio(output = AtCoder)]
fn main(n: usize, m: usize, s: String, t: String) -> usize {
    let mut dp = vec![0; (n + 1) * (m + 1)];
    s.chars().fold(1, |i, ch_s| {
        t.chars().fold(1, |j, ch_t| {
            if ch_s == ch_t {
                dp[i + j * (n + 1)] = dp[(i - 1) + (j - 1) * (n + 1)] + 1;
            } else {
                dp[i + j * (n + 1)] = max(dp[i + (j - 1) * (n + 1)], dp[(i - 1) + j * (n + 1)]);
            }
            j + 1
        });
        if i == n {
            println!("{:?}", dp);
            dp[(n + 1) * (m + 1) - 1]
        } else {
            println!("{} cycle : {:?}", i ,dp);
            i + 1
        }
    })
}
