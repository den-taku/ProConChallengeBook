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
fn dijkstra<T>(v: usize, e: usize, s: usize, mut d: Vec<T>, edge: &Vec<(usize, usize, T)>) -> Option<Vec<T>>
    where T: Ord + Copy + Default + Add<Output=T> + Bounded
{
    // make adjacency list
    let mut adj_list: Vec<Vec<DirectedCost<T>>> = vec![Vec::new(); v];
    for i in 0..e {
        let from = edge[e].0;
        let to = edge[e].1;
        let cost = edge[e].2;
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
    d[s] = T::default();
    visited[s] = true;
    let mut next_candidate = BinaryHeap::new();
    while !(adj_list[s].is_empty()) {
        next_candidate.push(adj_list[s].pop());
    }



    None
}

#[argio(output = AtCoder)]
fn main() {
    println!("Hello, world!");
}
