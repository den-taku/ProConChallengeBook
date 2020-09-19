use competitive::prelude::*;

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
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
fn main(n: usize, k: usize, imf: [(usize, usize, usize);k]) -> i32 {
    let mut animal = UnionFind::new(n * 3);
    let mut ans = 0;
    for i in 0..k {
        let t = imf[i].0;
        let x = imf[i].1 - 1;
        let y = imf[i].2 - 1;

        if x < 0 || n <= x || y < 0 || n <= y {
            ans += 1;
            continue;
        }
        if t == 1 {
            if animal.same(x, y + n) || animal.same(x, y + 2 * n) {
                ans += 1;
            } else {
                animal.unite(x, y);
                animal.unite(x + n, y + n);
                animal.unite(x + 2 * n, y + 2 * n);
            }
        } else {
            if animal.same(x, y) || animal.same(x, y + 2 * n) {
                ans += 1;
            } else {
                animal.unite(x, y + n);
                animal.unite(x + n, y + 2 * n);
                animal.unite(x + 2 * n, y);
            }
        }
    }
    ans
}
