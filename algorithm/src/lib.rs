// binary_search ****************************************
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
// end binary_search **************************************************

// BinaryHeap **************************************** 
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

// end BinaryHeap ********************************************* 

// UnionFind **************************************************
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: { 
                let mut v = Vec::with_capacity(n);
                for i in 0..n {
                    v.push(i);
                }
                v
            },
            rank: vec![0;n]
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.find(self.par[x])
        }
    }
    
    fn unite(&mut self, x: usize, y: usize) {
        let x_par = self.find(x);
        let y_par = self.find(y);
        if x_par == y_par {
            return ();
        }
        if self.rank[x_par] < self.rank[y_par] {
            self.par[x_par] = y_par;
        } else {
            self.par[y_par] = x_par;
            if self.rank[x_par] == self.rank[y_par] {
                self.rank[x_par] += 1;
            }
        }
    }

    fn same(&self, x:usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

// end UnionFind ****************************************

// bellman_ford **************************************************
use num_traits::bounds::Bounded;
use std::ops::Add;

// v: vertex size
// e: edge size
// s: start index
// g: teminal index
// edge: all edge (from, to, cost) * not need (i, j, c) and (j, i, c) both
// T: cost's type
// when graph doesn't have negative cycles, this function returns Some(d).
unsafe fn bellman_ford<T>(v: usize, e: usize, s: usize, mut d: Vec<T>, edge: &Vec<(usize, usize, T)>) -> Option<Vec<T>>
    where T: Ord + Bounded + Add<Output=T> + Default + Copy
{
    d.clear();
    for _ in 0..v {
        d.push(T::max_value());
    }
    d[s] = T::default();
    let mut all_edge_pattern = edge.clone();
    all_edge_pattern.extend(edge.iter().map(|elem| (elem.1, elem.0, elem.2)));
    for i in 0..v {
        let mut update = false;
        for j in 0..2 * e {
            let e_tmp = &all_edge_pattern[j];
            if d[e_tmp.0] != T::max_value() && d[e_tmp.1] > d[e_tmp.0] + e_tmp.2 {
                d[e_tmp.1] = d[e_tmp.0] + e_tmp.2;
                update = true;
            }
        }
        if !update {
            break;
        }
        if i == v - 1 {
            return None;
        }
    }
    Some(d)
}
// end bellman_ford ****************************************

// dijkstra **************************************************
use std::collections::BinaryHeap;
use num_traits::bounds::Bounded;
use std::ops::Add;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq)]
struct DirectedCost<T: Ord> (usize, T);

impl<T> Ord for DirectedCost<T> 
    where T: PartialEq + Eq + Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
            .then_with(||self.0.cmp(&other.0))
    }
}

impl<T> PartialOrd for DirectedCost<T> 
    where T: PartialEq + Eq + Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// signature's refernce -> bellman_ford
fn dijkstra<T>(v: usize, e: usize, s: usize, g: usize, mut d: Vec<T>, edge: &Vec<(usize, usize, T)>) -> Option<Vec<T>>
    where T: Ord + Copy + Default + Add<Output=T> + Bounded
{
    // make adjacency list
    let mut adj_list: Vec<Vec<DirectedCost<T>>> = vec![Vec::new(); v];
    for i in 0..e {
        let from = edge[i].0;
        let to = edge[i].1;
        let cost = edge[i].2;
        adj_list[from].push(DirectedCost(to, cost));
        adj_list[to].push(DirectedCost(from, cost));
    }
    // `visited` know either the vertex is visited or not
    let mut visited = vec![false; v];
    // shortest path's cost
    d.clear();
    for i in 0..v {
        d.push(T::max_value());
    }
    let mut next_candidates = BinaryHeap::new();
    next_candidates.push(DirectedCost(s, T::default()));
    let mut root_cost = T::default();
    while let Some(DirectedCost(next_position, next_cost)) = next_candidates.pop() {
        visited[next_position] = true;
        if next_position == g {
            d[next_position] = next_cost;
            return Some(d);
        }
        if next_cost > d[next_position] {
            continue;
        }
        for ed in &adj_list[next_position] {
            let next_edge = DirectedCost(ed.0, next_cost + ed.1);
            if next_edge.1 < d[next_edge.0] {
                d[next_edge.0] = next_edge.1;
                next_candidates.push(next_edge);
            }
        }
    }
    None
}
// end dijkstra **************************************************



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
