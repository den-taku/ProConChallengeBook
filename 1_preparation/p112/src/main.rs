use competitive::prelude::*;

fn binary_search<T: Ord>(n: usize, x: T, p: &[T]) -> Option<usize>{
    let mut l = 0;
    let mut r = n;

    while r - l >= 1 {
        let i = ( l + r ) / 2;
        if p[i] == x {
            return Some(i);
        } else if p[i] < x {
            l = i + 1;
        } else {
            r = i;
        }
    }

    None
}

#[argio(output = AtCoder)]
fn main(n: usize, m: i64, mut a: [i64; n]) -> bool {
    let mut v = vec![0; n * n];

    for i in 0..n {
        for j in 0..n {
            v[i * n + j] = a[i] + a[j];
        }
    }

    v.sort();

    for i in 0..n {
        for j in 0..n {
            if let Some(_) = binary_search(n, m - a[i] - a[j], &v) {
                return true;
            }
        }
    }

    // for i in 0..n {
    //     for j in 0..n {
    //         for k in 0..n {
    //             if let Some(_) = binary_search(n, m - a[i] - a[j] - a[k], &a) {
    //                 return true;
    //             }
    //         }
    //     }
    // }

    false
}
