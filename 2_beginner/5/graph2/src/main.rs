use competitive::prelude::*;
use std::collections::HashSet;

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


#[argio(output = AtCoder)]
fn main(n: usize, e: usize, edge: [(usize, usize); e]) -> bool {
    let mut node = vec![Vec::<usize>::new(); n];
    for i in 0..e {
        node[edge[i].0].push(edge[i].1);
    }
    let mut color = UnionFind::new(2 * n);
    for i in 0..e {
        while let Some(v) = node[i].pop() {
            if color.same(i, v) || color.same(i + n, v + n) {
                return false;
            } else {
                color.unite(i, v + n);
                color.unite(i + n, v);
            }
        }
    }
    true
}





































































































