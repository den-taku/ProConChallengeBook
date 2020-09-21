use competitive::prelude::*;
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

#[argio(output = AtCoder)]
fn main(v: usize, e: usize, s: usize, g: usize, edge: [(usize, usize, i32); e]) -> i32 {
    let mut d = Vec::<i32>::new();
    match dijkstra(v, e, s, g, d, &edge) {
        Some(d) => d[g],
        None => -1
    }
}
