use competitive::prelude::*;
use std::collections::VecDeque;

const INF: i64 = 100_000_000;
const dx: [isize; 4] = [1, 0, -1, 0];
const dy: [isize; 4] = [0, 1, 0, -1];

#[derive(Copy, Clone, Debug)]
struct P (usize, usize);

fn bfs(N: usize, M: usize, mut field: Vec<char>, sx: usize, sy: usize, gx: usize, gy: usize) -> i64{
    let mut d = vec![INF; N*M];
    let mut que = VecDeque::new();
    // for i in 0..N {
    //     for j in 0..M {
    //         d[i + j * M] = INF;
    //     }
    // }
    que.push_back(P(sx, sy));
    d[sx + sy * M] = 0;
    while !que.is_empty() {
        let p = que.pop_front().expect("que returns `None`");
        if p.0 == gx && p.1 == gy {
            break;
        }
        for i in 0..4 {
            let nx = p.0 as isize + dx[i];
            let ny = p.1 as isize + dy[i];
            if 0 <= nx && nx <= N as isize && 0 <= ny && ny <= M as isize && field[nx as usize + ny as usize * M] != '#' &&d[nx as usize + (ny as usize) * M] == INF {
                que.push_back(P(nx as usize, ny as usize));
                d[nx as usize + (ny as usize) * M] = d[p.0 + p.1 * M] + 1;
            }
        }
    }
    d[gx + gy * M]
}

#[argio(output = AtCoder)]
fn main(N: usize, M: usize, mut field: [char; N * M]) -> i64{
    bfs(N, M, field, 0, 0, M, N)
}
