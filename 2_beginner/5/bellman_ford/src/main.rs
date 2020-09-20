use num_traits::bounds::Bounded;
use std::ops::Add;
use competitive::prelude::*;

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

#[argio(output = AtCoder)]
fn main(v: usize, e: usize, s: usize, g: usize, edge: [(usize, usize, i32); e]) -> i32 {
    let d = Vec::<i32>::new();
    match unsafe { bellman_ford(v, e, s, d, &edge) } {
        Some(ans) => ans[g],
        None => i32::min_value()
    }
}
