use competitive::prelude::*;
use std::cmp::max;
use std::cmp::min;

#[argio(output = AtCoder)]
fn main(L: i64, n: usize, x: [i64; n]) {
    let mut minT = 0;
    let mut maxT = 0;

    for i in 0..n {
        minT = max(minT, min(x[i], L - x[i]));
        maxT = max(maxT, max(x[i], L - x[i]));
    }
    println!("min = {}", minT);
    println!("max = {}", maxT);
}
