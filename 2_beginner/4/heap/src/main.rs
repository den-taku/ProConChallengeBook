#[derive(Debug, Clone)]
struct BinaryHeap<T:Ord> {
    array: Vec<T>
}

// use these method for secure index
impl<T> Binary<T> for Vec<T>
    where T: Clone
{
    unsafe fn get_self(&self, index: usize) -> T {
        match self.get(index) {
            Some(r) => (*r).clone(),
            None => unreachable!()
        }
    }

    unsafe fn get_parant(&self, index: usize) -> T {
        match self.get((index - 1) / 2) {
            Some(r) => (*r).clone(),
            None => unreachable!()
        }
    }

    unsafe fn get_left_child(&self, index: usize) -> T {
        match self.get(2 * index + 1) {
            Some(r) => (*r).clone(),
            None => unreachable!()
        }
    }

    unsafe fn get_right_child(&self, index: usize) -> T {
        match self.get(2 * index + 2) {
            Some(r) => (*r).clone(),
            None => unreachable!()
        }
    }
}

trait Binary<T> {
    unsafe fn get_self(&self, index: usize) -> T;
    unsafe fn get_parant(&self, index: usize) -> T;
    unsafe fn get_left_child(&self, index: usize) -> T;
    unsafe fn get_right_child(&self, index: usize) -> T;
}

impl<T> BinaryHeap<T> 
    where T: Ord + Default + Clone,
{
    fn new() -> BinaryHeap<T> {
        BinaryHeap { array: Vec::<T>::new() }
    }

    fn push(&mut self, x: T) {
        self.array.push(x);
        self.orderize_from_down(self.array.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.array.len() == 0 {
            return None;
        }
        let last_index = self.array.len() - 1;
        self.array.swap(0, last_index);
        let value = self.array.pop();
        self.orderize_from_top(0);
        value
    }
            
     fn orderize_from_down(&mut self, start: usize) {
        let mut i = start;
        while i > 0 {
            let parant;
            let child;
            unsafe {
                parant = self.array.get_parant(i);
                child = self.array.get_self(i);
            }
            if parant > child {
                self.array.swap((i - 1) / 2, i);
                i = (i - 1) / 2;
            } else {
                break;
            }
        }
    }

    // ZZZZ
    fn orderize_from_top(&mut self, start: usize) {
        if self.array.len() != 0 {
            let mut i = start;
            let l = self.array.len();
            loop {
                let mut c_value: [T; 2] = [T::default(), T::default()];
                if 2 * i + 1 < l {
                    unsafe {
                        c_value[0] = self.array.get_left_child(i);
                    }
                } else {
                    unsafe {
                        c_value[0] = self.array.get_self(i);
                    }
                }
                if 2 * i + 2 < l {
                    unsafe {
                        c_value[1] = self.array.get_right_child(i);
                    }
                } else {
                    unsafe {
                        c_value[1] = self.array.get_self(i);
                    }
                }
                let parant;
                unsafe {
                    parant = self.array.get_self(i);
                }
                if c_value[0].clone() <= c_value[1].clone() {
                    if parant > c_value[0] {
                        self.array.swap(i, 2 * i + 1);
                        i = 2 * i + 1;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    if parant > c_value[1] {
                        self.array.swap(i, 2 * i + 2);
                        i = 2 * i + 2;
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }
           
}


// if parant > left {
//                     self.array.swap(i, 2 * i + 1);
//                     i = 2 * i + 1;
//                     continue;
//                 } else 

#[test]
fn test_push() {
    let bh = BinaryHeap::new();
    let mut bhc = bh.clone();
    for i in (0..11).rev() {
        bhc.push(i);
    }
}

#[test]
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
    let mut v = Vec::new();
    for i in 0..11 {
        match bh.pop() {
            Some(e) => v.push(e),
            None => println!("No more: {}", i)
        }
        println!("{:?}", bh)
    }        
    println!("{:?}", bh);
    println!("{:?}", v);
}
