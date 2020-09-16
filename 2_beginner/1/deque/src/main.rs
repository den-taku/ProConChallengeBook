use std::collections::VecDeque;

fn main() {
    let mut dq = VecDeque::new();
    dq.push_front(1);
    dq.push_back(2);
    println!("{:?}", dq);
}
