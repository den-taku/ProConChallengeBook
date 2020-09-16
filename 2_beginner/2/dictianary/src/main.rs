use competitive::prelude::*;
use std::collections::VecDeque;

#[argio(output = AtCoder)]
fn main(N: usize, mut S: [char; N]) -> String{
    let mut a = 0;
    let mut b = N - 1;
    let mut T = String::new();
    while a <= b {
        let mut left = false;
        let mut i = 0;
        while a + i <= b {
            if S[a + i] < S[b - i] {
                left = true;
                break;
            } else if S[a + i] > S[b - i] {
                left = false;
                break;
            }
            i += 1;
        }
        if left {
            T.push(S[a]);
            a += 1;
        } else {
            T.push(S[b]);
            b -= 1;
        }
    }
    T

}

//     let mut deque = VecDeque::from(S);
//     let mut T = String::new();
//     while !deque.is_empty() {
//         if deque.front() < deque.back() {
//             T.push(deque.pop_front().expect("pop_front_error"));
//         } else if deque.front() > deque.back() {
//             T.push(deque.pop_back().expect("pop_back_error"));
//         } else {
//             let mut front = String::new();
//             front.push(deque.pop_front().expect("1"));
//             let mut back = String::new();
//             back.push(deque.pop_back().expect("2"));
//             while deque.front() == deque.back() && !deque.is_empty() && !deque.capacity() == 1{
//                 front.push(deque.pop_front().expect("3"));
//                 back.push(deque.pop_back().expect("4"));
//             }
//             if deque.capacity() == 1 {
//                 while !front.is_empty() {
//                     T.push(front);
//                     T.push(back);
//                 }
//                 T.push(deque.pop_back().expect("5"));
//                 break;
//             } else if deque.is_empty() {
//                 T.push_str(&front);
//                 T.push_str(&back);
//                 break;
//             } if deque.front() < deque.back() {
//                 T.push(front.pop().expect("6"));
// 
//         }
//     }
// 
//     T
// 
