use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(N: usize, R: i64, mut X:[i64; N]) -> i64 {
    let mut ans = 0;
    let mut j = 0;
    for mut i in 1..N {
        if X[j] + R < X[i] {
            ans += 1;
            println!("i: {}, j: {}", i, j);
            'a:
            for k in i..N {
                if X[i - 1] + R < X[k] {
                    j = k;
                    println!("X[k] : {}", X[k]);
                    while i == k - 1 {
                        i += 1;
                    }
                    break 'a;
                }
            }
        }
    }
    println!("j:{}", j);


    ans + 1
}
