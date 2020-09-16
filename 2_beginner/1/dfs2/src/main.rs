use competitive::prelude::*;
use std::ops::IndexMut;

fn dfs(x: usize, y: usize, N: usize, M: usize, mut field: Vec<char>) -> Vec<char> {
    *field.index_mut(x + y * M) = '.';

    for dx in -1..2 {
        for dy in -1..2 {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if 0 <= nx && nx <= N && 0 <= ny && ny <= M && *field.index_mut(nx + ny * M) == 'W' {
                field = dfs(nx, ny, N, M, field);
            }
        }
    }
    field
}

#[argio(output = AtCoder)]
fn main(N: usize, M: usize, mut field: [char; N * M]) -> i64{
    let mut res: i64 = 0;
    for i in 0..N {
        for j in 0..M {
            if field[i + j * M] == 'W' {
                field = dfs(i, j, N, M, field);
                res += 1;
            }
        }
    }
    res
}
