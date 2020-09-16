
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




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
