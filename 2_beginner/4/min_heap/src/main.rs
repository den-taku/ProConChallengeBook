use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut mini_bh = BinaryHeap::new();
    for i in (0..11).rev() {
        mini_bh.push(Reverse(i));
    }
    println!("{:?}", mini_bh);
    let mut neg_bh = BinaryHeap::new();
    for i in (0..11).rev() {
        neg_bh.push(-(i - 4));
    }
    println!("{:?}", neg_bh);
}
