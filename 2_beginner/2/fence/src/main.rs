use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(mut N: usize, mut L: [i64; N]) -> i64 {
    let mut ans = 0;
    while N > 1 {
        let mut mil1 = 0;
        let mut mil2 = 1;
        if L[mil1] > L[mil2] {
            L.swap(mil1, mil2);
        }
        for i in 2..N {
            if L[i] < L[mil1] {
                mil2 = mil1;
                mil1 = i;
            } else if L[i] < L[mil2] {
                mil2 = i;
            }
        }
        let tmp = L[mil1] + L[mil2];
        ans += tmp;

        if mil1 == N - 1 {
            L.swap(mil1, mil2);
        }
        L[mil1] = tmp;
        L[mil2] = L[N - 1];
        N -= 1;
    }
    ans
}
