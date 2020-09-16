use competitive::prelude::*;

fn dfs(i: usize, sum: i64, n: usize, k: i64, a: &[i64]) -> bool {
    if i == n {
        return sum == k;
    }
    if dfs(i+1, sum, n, k, &a) {
        return true;
    }
    if dfs(i+1, sum + a[i], n, k, &a) {
        return true;
    }
    false
}

#[argio(output = AtCoder)]
fn main(n: usize, a: [i64; n], k: i64) -> bool {
    dfs(0, 0, n, k, &a)
}
