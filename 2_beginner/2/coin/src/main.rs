use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(mut c_1: i64, mut c_5: i64, mut c_10: i64, mut c_50: i64, mut c_100: i64, mut c_500: i64, mut A: i64) -> i64 {
    let mut ans = 0;
    while A >= 500 && c_500 > 0 {
        A -= 500;
        c_500 -= 1;
        ans += 1;
    }
    while A >= 100 && c_100 > 0 {
        A -= 100;
        c_100 -= 1;
        ans += 1;
    }
    while A >= 50 && c_50 > 0 {
        A -= 50;
        c_50 -= 1;
        ans  += 1;
    }
    while A >= 10 && c_10 > 0 {
        A -= 10;
        c_10 -= 1;
        ans += 1;
    }
    while A >= 5 && c_5 > 0 {
        A -= 5;
        c_5 -= 1;
        ans += 1;
    }
    while A >= 1 && c_1 > 0 {
        A -= 1;
        c_1 -= 1;
        ans  += 1;
    }
    ans
}
