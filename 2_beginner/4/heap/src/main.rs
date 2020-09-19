#[derive(Debug, Clone)]
struct BinaryHeap<T:Ord> {
    array: Vec<T>
}

impl<T> BinaryHeap<T> 
    where T: Ord + Default + Clone
{
    fn new() -> BinaryHeap<T> {
        BinaryHeap { array: Vec::<T>::new() }
    }

    fn orderize(&mut self, start: usize) {
        let mut i = start;;
        while i > 0 {
            let parant = match self.array.get((i - 1) / 2) {
                Some(r) => (*r).clone(),
                None => T::default()
            };
            let child = match self.array.get(i) {
                Some(r) => (*r).clone(),
                None => T::default()
            };
            if parant > child {
                self.array.swap((i - 1) / 2, i);
                i = (i - 1) / 2;
            } else {
                break;
            }
        }
    }

    fn push(&mut self, x: T) {
        self.array.push(x);
        self.orderize(self.array.len() - 1);
    }

    // fn pop(&mut self) -> T {
    // }
            
            
}

#[test]
fn test_push() {
    let bh = BinaryHeap::new();
    let mut bhc = bh.clone();
    for i in (0..11).rev() {
        bhc.push(i);
    }
}

fn test_orderize() {
    let mut bh = BinaryHeap::new();
    let mut index = 0;
    for i in (0..11).rev() {
        bh.push(i);
        bh.orderize(index);
        index += 1;
    }

}


fn main() {
    let mut bh = BinaryHeap::new();
    for i in (0..11).rev() {
        bh.push(i);
    }
    println!("{:?}", bh);
}
