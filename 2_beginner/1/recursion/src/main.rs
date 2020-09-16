fn fact(n: i64) -> i64 {
    if n <= 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn fib(n: usize) -> i64 {
    if n <= 1 {
        n as i64
    } else {
        fib(n - 1) + fib(n - 2) as i64
    }
}

// This cannot work because of mutalbe referense constraint

// fn fib(n: usize, buf: &mut [i64]) -> i64 {
//     if n <= 1 {
//         n as i64
//     } else if buf[n] != 0 {
//         buf[n] 
//     } else {
//         buf[n] = fib(n - 1, &mut buf) + fib(n - 2, &mut buf);
//         buf[n]
//     }
// }

fn main() {
    println!("fact(20) = {}", fact(20));
    // let mut a = [0_i64; 9];
    println!("fib(8) = {}", fib(8));
}
