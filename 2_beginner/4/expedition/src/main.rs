use competitive::prelude::*;
use std::collections::BinaryHeap;

#[argio(output = AtCoder)]
fn main(n: usize, l: usize, mut p: i64, mut a: [usize; n], b: [i64; n]) -> i64{
    let mut stand = BinaryHeap::new();
    let mut next_pos = 0;
    let mut ans = 0;
    for i in 1..l {
        p -= 1;
        if i == a[next_pos] {
            stand.push(b[next_pos]);
            next_pos += 1;
            if next_pos == n {
                next_pos -= 1;
                a[n-1] = 0
            }
        }
        if p == 0 {
            if let Some(gas) = stand.pop() {
                p += gas;
                ans += 1;
            } else {
                return -1;
            }
        }
    }
    ans
}
