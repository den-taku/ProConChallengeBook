use competitive::prelude::*;
// use std::cmp::max;

#[argio(output = AtCoder)]
fn main(n: usize, _goods: [(i64, i64); n], w: i64) -> i64{
    let mut _dp = vec![0; (n + 1)  * (w as usize + 1)];
    // dp[0][j] = 0
    // dp[i+1][j] = max(dp[i][j], dp[i+1][j-w[i]]+v[i])
    0
}
