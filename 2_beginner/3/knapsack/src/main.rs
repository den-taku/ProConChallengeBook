use competitive::prelude::*;
use std::cmp::max;

trait Compare {
    fn compare(&self, i: usize, n: i64, buf: &mut Vec<i64>);
}

impl Compare for Vec<i64> {
    fn compare(&self, i: usize, n: i64, buf: &mut Vec<i64>) {
        buf[i] = max(self[i], n);
    }
}

#[argio(output = AtCoder)]
fn main(n: usize, goods: [(i64, i64); n], w: i64) -> i64{
    let mut v = vec![0; w as usize + 1];
    for i in 0..n {
        let mut v_buf = v.clone();
        for j in 0..w+1 {
            if 0 <= j - goods[i].0 {
                v.compare(j as usize, v[j as usize - goods[i].0 as usize] + goods[i].1, &mut v_buf);
            }
        }
        v = v_buf;
    }
    let mut ans = 0;
    for i in 0..(w as usize + 1) {
        ans = max(ans, v[i]);
    }
    println!("{:?}", v);
    ans
}
